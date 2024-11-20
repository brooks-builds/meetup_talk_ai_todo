use colored::Colorize;
use derive_more::derive::Display;

pub fn loggit(message: impl ToString, level: LogLevel) {
    if level == LogLevel::Normal {
        println!("{}", message.to_string().purple());

        return;
    }

    if level == LogLevel::Error {
        #[cfg(feature = "log_error")]
        println!("{}", message.to_string().red());
    }

    if level == LogLevel::Info {
        #[cfg(feature = "log_info")]
        println!("{}", message.to_string().blue());
    }

    if level == LogLevel::Debug {
        #[cfg(feature = "log_debug")]
        println!("{}", message.to_string().yellow());
    }
}

#[derive(Debug, PartialEq, Eq, Display)]
pub enum LogLevel {
    Normal,
    Error,
    Info,
    Debug,
}
