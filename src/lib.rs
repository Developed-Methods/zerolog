mod level;
pub(crate) mod spans;
mod logs;
mod log_attr;

pub use level::LogLevel;
pub use spans::{Span, SpanBuilder, set_min_log_level, disable_logging};
pub use logs::Log;

pub(crate) fn trim_src_path(path: &str) -> &str {
    if let Some((_, result)) = path.split_once("/src/") {
        return result;
    }
    if let Some(result) = path.strip_prefix("src/") {
        return result;
    }
    path
}

pub(crate) fn trim_fn_path(path: &str) -> &str {
    if let Some((_, result)) = path.rsplit_once("::") {
        return result;
    }
    path
}
