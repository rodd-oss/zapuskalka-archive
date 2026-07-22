use std::sync::Arc;

use tauri::async_runtime::spawn;
use tokio::{
    process::Child,
    sync::{broadcast, broadcast::error::RecvError, oneshot, Mutex},
};

// Events are tiny Copy values; the capacity only needs to cover a burst of
// terminations so slow receivers do not lag out and miss events.
const EVENT_CHANNEL_CAPACITY: usize = 64;

pub struct ProcessMonitor {
    events_channel: broadcast::Sender<ProcessEvent>,
    processes: Arc<Mutex<Vec<ProcessEntry>>>,
}

impl ProcessMonitor {
    pub fn new() -> Self {
        let (tx, mut rx) = broadcast::channel(EVENT_CHANNEL_CAPACITY);

        let processes: Arc<Mutex<Vec<ProcessEntry>>> = Arc::new(Mutex::new(vec![]));
        let procs_for_cleanup = processes.clone();

        // cleanup listener
        spawn(async move {
            loop {
                match rx.recv().await {
                    Ok(ProcessEvent::Terminated(pid)) => {
                        _ = Self::remove_process(procs_for_cleanup.clone(), pid).await;
                    }
                    Err(RecvError::Lagged(_)) => continue,
                    Err(RecvError::Closed) => break,
                }
            }
        });

        Self {
            events_channel: tx,
            processes,
        }
    }

    /// Subscribe to process termination events
    pub fn subscribe(&self) -> broadcast::Receiver<ProcessEvent> {
        self.events_channel.subscribe()
    }

    /// Add process to monitoring
    pub async fn add(&mut self, pid: u32, process: Child) {
        let (m2w_tx, m2w_rx) = oneshot::channel();

        spawn(Self::child_close_waiter(
            pid,
            process,
            m2w_rx,
            self.events_channel.clone(),
        ));

        let mut procs = self.processes.lock().await;
        procs.push(ProcessEntry { pid, tx: m2w_tx });
    }

    /// Terminate and remove process from monitoring
    pub async fn terminate(&mut self, pid: u32) -> Result<(), Error> {
        match Self::remove_process(self.processes.clone(), pid).await {
            Ok(tx) => {
                _ = tx.send(Monitor2Waiter::TerminateChild);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    async fn remove_process(
        procs: Arc<Mutex<Vec<ProcessEntry>>>,
        pid: u32,
    ) -> Result<oneshot::Sender<Monitor2Waiter>, Error> {
        let mut procs = procs.lock().await;
        let proc = procs.iter().enumerate().find(|(_, it)| it.pid == pid);
        if let Some((idx, _)) = proc {
            let entry = procs.swap_remove(idx);
            Ok(entry.tx)
        } else {
            Err(Error::ProcessNotFound)
        }
    }

    async fn child_close_waiter(
        pid: u32,
        mut child: Child,
        rx: oneshot::Receiver<Monitor2Waiter>,
        tx: broadcast::Sender<ProcessEvent>,
    ) {
        tokio::select! {
            res = child.wait() => {
                if let Err(e) = res {
                    eprintln!("Error waiting for process {}: {}", pid, e);
                }
            }

            msg = rx => {
                match msg {
                    Ok(Monitor2Waiter::TerminateChild) => {
                        if let Err(e) = child.kill().await {
                            eprintln!("Error during terminating process {}: {}", pid, e);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error during receiving channel message: {}", e);
                    }
                }
                // Whether or not the kill went through, keep reaping the child
                // so a Terminated event is always emitted below.
                if let Err(e) = child.wait().await {
                    eprintln!("Error waiting for process {}: {}", pid, e);
                }
            }
        }

        _ = tx.send(ProcessEvent::Terminated(pid));
    }
}

impl Default for ProcessMonitor {
    fn default() -> Self {
        Self::new()
    }
}

struct ProcessEntry {
    pub pid: u32,
    pub tx: oneshot::Sender<Monitor2Waiter>,
}

#[derive(Debug)]
pub enum Error {
    ProcessNotFound,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ProcessEvent {
    Terminated(u32),
}

enum Monitor2Waiter {
    TerminateChild,
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::process::Command;
    use tokio::time::timeout;

    fn spawn_sleeper() -> Child {
        Command::new("sleep")
            .arg("30")
            .spawn()
            .expect("failed to spawn sleep")
    }

    fn spawn_short_lived() -> Child {
        Command::new("true").spawn().expect("failed to spawn true")
    }

    async fn expect_terminated(rx: &mut broadcast::Receiver<ProcessEvent>, pid: u32) {
        loop {
            match timeout(Duration::from_secs(5), rx.recv()).await {
                Ok(Ok(ProcessEvent::Terminated(p))) if p == pid => return,
                Ok(Ok(_)) => continue,
                Ok(Err(RecvError::Lagged(_))) => continue,
                Ok(Err(e)) => panic!("event channel error: {}", e),
                Err(_) => panic!("timed out waiting for Terminated({})", pid),
            }
        }
    }

    #[tokio::test]
    async fn natural_exit_emits_terminated() {
        let mut monitor = ProcessMonitor::new();
        let mut rx = monitor.subscribe();

        let child = spawn_short_lived();
        let pid = child.id().expect("child has no pid");
        monitor.add(pid, child).await;

        expect_terminated(&mut rx, pid).await;

        // The cleanup listener runs on another task; give it a moment.
        tokio::time::sleep(Duration::from_millis(200)).await;
        assert!(monitor.processes.lock().await.is_empty());
    }

    #[tokio::test]
    async fn terminate_kills_and_emits() {
        let mut monitor = ProcessMonitor::new();
        let mut rx = monitor.subscribe();

        let child = spawn_sleeper();
        let pid = child.id().expect("child has no pid");
        monitor.add(pid, child).await;

        monitor.terminate(pid).await.expect("terminate failed");
        expect_terminated(&mut rx, pid).await;
    }

    #[tokio::test]
    async fn terminate_unknown_pid_errors() {
        let mut monitor = ProcessMonitor::new();
        assert!(matches!(
            monitor.terminate(999_999_999).await,
            Err(Error::ProcessNotFound)
        ));
    }

    #[tokio::test]
    async fn lagged_receiver_recovers() {
        let mut monitor = ProcessMonitor::new();
        let mut rx = monitor.subscribe();

        // Overflow the channel while rx is not being polled so it lags.
        for i in 0..(EVENT_CHANNEL_CAPACITY + 8) {
            _ = monitor
                .events_channel
                .send(ProcessEvent::Terminated(1_000_000 + i as u32));
        }

        let child = spawn_sleeper();
        let pid = child.id().expect("child has no pid");
        monitor.add(pid, child).await;
        monitor.terminate(pid).await.expect("terminate failed");

        // Despite lagging, the receiver must still see the real event,
        // and the internal cleanup listener must still be alive.
        expect_terminated(&mut rx, pid).await;
    }
}
