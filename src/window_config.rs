// global config file. in here the config will be parsed to the system-specific config

use crate::mac_window::NS_BACKING_STORE_BUFFERED;

/// the global window config
pub struct WindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub position: (f64, f64),
    pub backing: u8,
    pub has_title: bool,
    pub closable: bool,
    pub resizable: bool,
    pub minimizable: bool,
}

/// default values for the window config
impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: String::from("My App"),
            width: 800,
            height: 600,
            backing: NS_BACKING_STORE_BUFFERED,
            position: (0f64,0f64),
            has_title: true,
            closable: true,
            resizable: true,
            minimizable: true,
        }
    }
}

/// the system-specific types and parameters for macOS
pub struct MacosWindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub position: (f64, f64),
    pub backing: u8,
    pub has_title: bool,
    pub closable: bool,
    pub resizable: bool,
    pub minimizable: bool,
}

/// converts the window config to the system-specific config for macOS
pub fn macos_config (window_config: &WindowConfig) -> MacosWindowConfig {
    MacosWindowConfig{
        title: window_config.title.clone(),
        width: window_config.width,
        height: window_config.height,
        position: window_config.position,
        backing: window_config.backing,
        has_title: window_config.has_title,
        closable: window_config.closable,
        resizable: window_config.resizable,
        minimizable: window_config.minimizable,
    }
}