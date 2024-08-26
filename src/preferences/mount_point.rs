use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Serialize, Deserialize, Debug, Tabled)]
pub struct MountPoint {
    pub name: String,
    pub address: String,
    pub mount_location: String,
    pub flags: String,

    #[tabled(display_with = "format_option_bool")]
    pub ask_for_password: Option<bool>,
}

fn format_option_bool(opt: &Option<bool>) -> String {
    match opt {
        Some(true) => "Yes".to_string(),
        _ => "No".to_string(),
    }
}
