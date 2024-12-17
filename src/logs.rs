use std::panic::Location;

use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::{log_attr::LogAttr, LogLevel, spans::SPAN_STACK};

pub struct Log<A: LogAttr> {
    time: DateTime<Utc>,
    span: u64,
    level: LogLevel,
    msg: &'static str,
    caller: &'static Location<'static>,
    pub caller_fn: Option<&'static str>,
    print: bool,
    attrs: Option<A>,
}

impl Log<()> {
    #[track_caller]
    pub fn new(level: LogLevel, msg: &'static str) -> Self {
        let caller = Location::caller();
        let span = SPAN_STACK.with_borrow(|s| *s.last().unwrap());

        Log {
            time: Utc::now(),
            span: span.id,
            level,
            msg,
            caller,
            caller_fn: None,
            print: span.do_print,
            attrs: Some(()),
        }
    }
}

impl<A: LogAttr> Log<A> {
    pub fn attr<T: Serialize>(mut self, name: &'static str, value: T) -> Log<impl LogAttr> {
        Log {
            time: self.time,
            span: self.span,
            level: self.level,
            msg: self.msg,
            caller: self.caller,
            caller_fn: self.caller_fn,
            print: self.print,
            attrs: self.attrs.take().map(|a| a.add_attr(name, value)),
        }
    }
}

impl<A: LogAttr> Drop for Log<A> {
    fn drop(&mut self) {
        println!("{{\
            \"ts\":{:?},\
            \"type\":\"log\",\
            \"span\":{:?},\
            \"level\":\"{}\",\
            \"msg\":{:?},\
            \"caller\":\"{}:{}\",\
            \"caller_fn\":{:?},\
            \"attrs\":{}\
            }}",
            self.time.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            self.span,
            self.level,
            self.msg,
            self.caller.file(), self.caller.line(),
            self.caller_fn.unwrap_or(""),
            serde_json::to_string(&self.attrs.take()).unwrap(),
        );
    }
}

#[macro_export]
macro_rules! log {
    ($level:expr, $name:expr) => {{
        let mut log = $crate::Log::new($level, $name);
        log.caller_fn = Some({
            fn f() {}
            fn type_name_of<T>(_: T) -> &'static str {
                std::any::type_name::<T>()
            }
            let name = type_name_of(f);

            // Find and cut the rest of the path
            &name[..name.len() - 3]
        });
        log
    }};
}
