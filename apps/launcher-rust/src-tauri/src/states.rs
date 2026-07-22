use std::{collections::HashMap, path::PathBuf, sync::Arc};

use tokio::sync::Mutex;

use crate::process_monitor::ProcessMonitor;

pub struct ConfigDirPath(pub PathBuf);

pub struct AppPidMapInner {
    pub appid2pid: HashMap<String, u32>,
    pub pid2appid: HashMap<u32, String>,
}

pub type AppPidMap = Arc<Mutex<AppPidMapInner>>;

pub type ProcessMonitorInstance = Arc<Mutex<ProcessMonitor>>;
