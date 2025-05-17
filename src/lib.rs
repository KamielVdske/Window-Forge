pub use crate::window_config::WindowConfig;

mod mac_window;
mod window_config;

#[cfg(target_os = "macos")]
pub fn create_window (window_config: &WindowConfig) -> u8 {
    mac_window::create_window(window_config)
}



#[cfg(target_os = "windows")]
pub fn create_window () -> u8 {

}
