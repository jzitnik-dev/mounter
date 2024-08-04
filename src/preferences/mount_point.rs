use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Serialize, Deserialize, Debug, Tabled)]
pub struct MountPoint {
    pub name: String,
    pub address: String,
    pub mount_location: String,
    pub flags: String,
}
