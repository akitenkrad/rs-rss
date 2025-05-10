use shared::logger::init_logger;

pub fn setup() {
    // Initialize the logger
    init_logger().expect("Failed to initialize logger");
}
