use std::{cell::RefCell, panic::Location, sync::atomic::{AtomicU64, Ordering}, time::Instant};
use chrono::Utc;
use serde::Serialize;

use crate::{level::LogLevel, log_attr::LogAttr};

static SPAN_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone, Copy)]
pub(crate) struct StackEntry {
    pub id: u64,
    pub do_print: bool,
}

thread_local! {
    pub(crate) static SPAN_STACK: RefCell<Vec<StackEntry>> = RefCell::new({
        let mut vec = Vec::with_capacity(32);
        vec.push(StackEntry {
            id: 0,
            do_print: true,
        });
        vec
    });
}

pub fn print_logging(enabled: bool) {
    SPAN_STACK.with_borrow_mut(|s| {
        s.last_mut().unwrap().do_print = enabled;
    });
}

pub struct SpanBuilder<A: LogAttr> {
    pub level: LogLevel,
    pub name: &'static str,
    pub caller_fn: Option<&'static str>,
    pub caller: &'static Location<'static>,
    attrs: A,
}

impl SpanBuilder<()> {
    #[track_caller]
    pub fn new(level: LogLevel, name: &'static str) -> Self {
        let caller = Location::caller();

        SpanBuilder {
            level,
            name,
            caller_fn: None,
            caller,
            attrs: (),
        }
    }
}

pub struct Span {
    id: u64,
    parent_id: u64,
    depth: u64,

    time: Instant,
    level: LogLevel,
    name: &'static str,
    caller_fn: Option<&'static str>,
    caller: &'static Location<'static>,

    printed: bool,
    _no_send: *mut u8,
}

impl<A: LogAttr> SpanBuilder<A> {
    pub fn attr<T: Serialize>(self, name: &'static str, value: T) -> SpanBuilder<impl LogAttr> {
        let check = A::CHECK;
        assert_eq!(check, 0);

        SpanBuilder {
            level: self.level,
            name: self.name,
            caller_fn: self.caller_fn,
            caller: self.caller,
            attrs: self.attrs.add_attr(name, value),
        }
    }

    pub fn build(self) -> Span {
        let id = SPAN_ID.fetch_add(1, Ordering::AcqRel);

        let (parent_id, depth, printed) = SPAN_STACK.with_borrow_mut(|stack| {
            let depth = stack.len();
            let last = *stack.last().unwrap();

            stack.push(StackEntry {
                id,
                do_print: last.do_print,
            });

            (last.id, depth as u64, last.do_print)
        });

        if printed {
            println!("{{\
                \"ts\":{:?},\
                \"type\":\"span\",\
                \"id\":{:?},\
                \"parent\":{:?},\
                \"depth\":{:?},\
                \"level\":\"{}\",\
                \"name\":{:?},\
                \"caller\":\"{}:{}\",\
                \"caller_fn\":{:?},\
                \"attrs\":{}\
                }}",
                Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
                id,
                parent_id,
                depth,
                self.level,
                self.name,
                self.caller.file(), self.caller.line(),
                self.caller_fn.unwrap_or(""),
                serde_json::to_string(&self.attrs).unwrap(),
            );
        }

        Span {
            id,
            parent_id,
            depth,
            time: Instant::now(),
            level: self.level,
            name: self.name,
            caller_fn: self.caller_fn,
            caller: self.caller,
            printed,
            _no_send: std::ptr::null_mut(),
        }
    }
}

impl Drop for Span {
    fn drop(&mut self) {
        SPAN_STACK.with_borrow_mut(|stack| {
            let tail = stack.pop().unwrap();
            assert_eq!(tail.id, self.id);
            assert_eq!(stack.last().unwrap().id, self.parent_id);
            assert_eq!(stack.len(), self.depth as usize);
        });

        if self.printed {
            let duration = self.time.elapsed();

            println!("{{\
                \"ts\":{:?},\
                \"type\":\"exit\",\
                \"id\":{:?},\
                \"parent\":{:?},\
                \"depth\":{:?},\
                \"duration_us\":{:?}\
                }}",
                Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
                self.id,
                self.parent_id,
                self.depth,
                duration.as_micros(),
            );
        }
    }
}

impl Span {
    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn time(&self) -> Instant {
        self.time
    }

    pub fn level(&self) -> LogLevel {
        self.level
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn caller_fn(&self) -> Option<&'static str> {
        self.caller_fn
    }

    pub fn caller_file(&self) -> &'static str {
        self.caller.file()
    }

    pub fn caller_lineno(&self) -> u32 {
        self.caller.line()
    }
}

#[macro_export]
macro_rules! span {
    ($level:expr, $name:expr) => {{
        let mut span = $crate::SpanBuilder::new($level, $name);
        span.caller_fn = Some({
            fn f() {}
            fn type_name_of<T>(_: T) -> &'static str {
                std::any::type_name::<T>()
            }
            let name = type_name_of(f);

            // Find and cut the rest of the path
            &name[..name.len() - 3]
        });
        span
    }};
}
