/// Logger struct implementation
///
/// This implementation provides methods to create a new Logger, add loggers, and set a fallback logger.
///
/// # Methods
/// - `new`: Creates a new Logger instance.
/// - `add_logger`: Adds a logger to the Logger instance.
/// - `set_fallback`: Sets a fallback logger for the Logger instance.
///
/// # Example
/// ```
/// use utils::logging::Logger;
/// use utils::logging::log::Log;
/// use std::io::Write;
///
/// let mut logger = Logger::new();
/// logger.add_logger(Box::new(Log::new(Box::new(std::io::stdout()))));
/// logger.set_fallback(Box::new(Log::new(Box::new(std::io::stderr()))));
/// ```
pub struct Logger {
    loggers: Vec<Box<dyn log::Log>>,
    fallback: Option<Box<dyn log::Log>>,
}

impl Logger {
    pub fn new() -> Logger {
        Logger {
            loggers: Vec::new(),
            fallback: None,
        }
    }

    /// add a logger
    /// # Arguments
    /// * `logger` - The logger to add
    /// # Example
    /// ```
    /// use utils::logging::Logger;
    /// use utils::logging::log::Log;
    /// use std::io::Write;
    /// let mut logger = Logger::new();
    /// logger.add_logger(Box::new(Log::new(Box::new(std::io::stdout())));
    /// ```
    pub fn add_logger(&mut self, logger: Box<dyn log::Log>) {
        self.loggers.push(logger);
    }

    /// set a fallback logger
    /// # Arguments
    /// * `fallback` - The fallback logger
    /// # Example
    /// ```
    /// use utils::logging::Logger;
    /// use utils::logging::log::Log;
    /// use std::io::Write;
    /// let mut logger = Logger::new();
    /// logger.set_fallback(Box::new(Log::new(Box::new(std::io::stderr())));
    /// ```
    pub fn set_fallback(&mut self, fallback: Box<dyn log::Log>) {
        self.fallback = Some(fallback);
    }
}

impl log::Log for Logger {
    fn enabled(&self, _: &log::Metadata) -> bool {
        return true;
    }

    fn log(&self, record: &log::Record) {
        let mut logged = false;

        for logger in &self.loggers {
            if logger.enabled(record.metadata()) {
                logger.log(record);
                logged = true;
            }
        }

        if !logged {
            if let Some(fallback) = &self.fallback {
                fallback.log(record);
            }
        }
    }

    fn flush(&self) {
        for logger in &self.loggers {
            logger.flush();
        }
        if let Some(fallback) = &self.fallback {
            fallaback.flush();
        }
    }
}
