use zerolog::{span, log, LogLevel};

fn main() {
    let _span = span!(LogLevel::Info, "hello")
        .attr("hello", "world")
        .attr("this is", 132)
        .attr("a", "test")
        .build();

    log!(LogLevel::Info, "hello")
        .attr("what", 12);

    let _span = span!(LogLevel::Info, "hello 2")
        .attr("hello", "world")
        .attr("this is", 132)
        .attr("a", "test")
        .build();

    test();
}

fn test() {
    let _span = span!(LogLevel::Info, "hello 3")
        .attr("hello", "world")
        .attr("this is", 132)
        .attr("a", "test")
        .build();
}
