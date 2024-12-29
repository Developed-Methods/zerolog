use std::{fmt::Display, str::FromStr};

use serde::{de::Visitor, Deserialize, Serialize};

#[derive(PartialEq, Eq, Serialize, Clone, Copy, Debug, PartialOrd, Ord)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Trace => write!(f, "trace"),
            Self::Debug => write!(f, "debug"),
            Self::Info =>  write!(f, "info"),
            Self::Warn =>  write!(f, "warn"),
            Self::Error => write!(f, "error"),
        }
    }
}

impl<'de> Deserialize<'de> for LogLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        deserializer.deserialize_str(LogLevelVisitor)
    }
}

struct LogLevelVisitor;

impl<'de> Visitor<'de> for LogLevelVisitor {
    type Value = LogLevel;
    
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: serde::de::Error {
        self.visit_str(&v)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
        LogLevel::from_str(v).map_err(|_| E::unknown_variant(v, &["trace", "debug", "info", "warn", "error"]))
    }

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "one of [trace, debug, info, warn, error]")
    }
}

#[derive(Debug)]
pub struct LogLevelParseError(pub String);

impl std::fmt::Display for LogLevelParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for LogLevelParseError {
}

impl FromStr for LogLevel {
    type Err = LogLevelParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.eq_ignore_ascii_case("trace") {
            Ok(LogLevel::Trace)
        }
        else if s.eq_ignore_ascii_case("debug") {
            Ok(LogLevel::Debug)
        }
        else if s.eq_ignore_ascii_case("info") {
            Ok(LogLevel::Info)
        }
        else if s.eq_ignore_ascii_case("warn") {
            Ok(LogLevel::Warn)
        }
        else if s.eq_ignore_ascii_case("error") {
            Ok(LogLevel::Error)
        } else {
            Err(LogLevelParseError(format!("unknown level: {:?}", s)))
        }
    }
}

