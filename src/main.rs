use window_native::create_window;
use window_native::WindowConfig;


fn main() {
    let window_config = WindowConfig {
        title: String::from("Hello World"),
        width: 1000,
        resizable: false,
        ..WindowConfig::default()
    };
    create_window(&window_config);
}
