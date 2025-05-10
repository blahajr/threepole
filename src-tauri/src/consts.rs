use std::time::Duration;

pub const TARGET_NAME: &str = "destiny2.exe";
pub const OVERLAY_POLL_INTERVAL: Duration = Duration::from_millis(200);
pub const APP_NAME: &str = "threepole";
pub const APP_VER: &str = env!("CARGO_PKG_VERSION");
pub const API_KEY: &str = env!("BUNGIE_API_KEY");
pub const API_PATH: &str = "https://www.bungie.net/Platform";
pub const NAMED_PIPE: &str = r"\\.\pipe\threepole-open";
pub const USER_AGENT: &str = concat!("threepole", "/", env!("CARGO_PKG_VERSION"));

pub const RAID_ACTIVITY_MODE: usize = 4;
pub const DUNGEON_ACTIVITY_MODE: usize = 82;
pub const STRIKE_ACTIVITY_MODE: usize = 18;
pub const LOSTSECTOR_ACTIVITY_MODE: usize = 87;

pub const RAID_ACTIVITY_HASH: usize = 2043403989;
