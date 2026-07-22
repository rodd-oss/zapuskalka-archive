use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstalledAppInfo {
    pub id: String,
    pub build_id: String,
    pub install_dir: PathBuf,
    pub storage_dir: PathBuf,
    pub entrypoint: String,
}
