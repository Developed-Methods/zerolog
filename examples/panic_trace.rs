use zerolog::{install_panic_hook, span, LogLevel};

fn main() {
    // Install the panic hook at application startup.
    // This will print the span trace when a panic occurs.
    install_panic_hook();

    let _span = span!(LogLevel::Info, "application_start").build();

    process_request();
}

fn process_request() {
    let _span = span!(LogLevel::Info, "process_request").build();

    handle_data();
}

fn handle_data() {
    let _span = span!(LogLevel::Debug, "handle_data").build();

    // This panic will trigger the span trace output,
    // showing the full context of nested spans.
    panic!("something went wrong!");
}
