use zerolog::{log, set_min_log_level, span, LogLevel};

fn main() {
    let _span = span!(LogLevel::Info, "hello")
        .attr("hello", "world")
        .attr("this is", 132)
        .attr("a", "test")
        .build();

    log!(LogLevel::Info, "hello")
        .attr("what", 12);

    set_min_log_level(LogLevel::Warn);

    let _span = span!(LogLevel::Info, "hello 2")
        .attr("hello", "world")
        .attr("this is", 132)
        .attr("a", "test")
        .build();

    test();

    log!(LogLevel::Info, "hello again")
        .attr("what", 12);
}

fn test() {
    let _span = span!(LogLevel::Info, "hello 3")
        .attr("hello", "world")
        .attr("this is", 132)
        .attr("a", "test")
        .build();

    log!(LogLevel::Error, "something bad")
        .attr("reason", "why not");

    set_min_log_level(LogLevel::Trace);

    log!(LogLevel::Trace, "a trace");
}
