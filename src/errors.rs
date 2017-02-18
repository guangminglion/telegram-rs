use std::fmt::Display;
use serde::{ser, de};

error_chain! {
    foreign_links {
        SystemTime(::std::time::SystemTimeError);
        Hyper(::hyper::Error);
        Io(::std::io::Error);
    }
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Error {
        Error::from(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Error {
        Error::from(msg.to_string())
    }
}
