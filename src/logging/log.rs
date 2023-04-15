use crate::logging::log_origin::LogOrigin;
use crate::logging::log_type::LogType;
use std::time::SystemTime;

/// A Log message to be logged by the Logging thread
pub struct Log {
    /// The time when the log was originally created
    time: String,

    /// The thread or plugin that made the log
    pub log_origin: LogOrigin,

    /// The urgency of the log
    pub log_type: LogType,

    /// The reason for the log
    pub description: String,
}

impl Log {
    /// Assemble a new log with the given origin, type and description
    ///
    /// # Arguments
    ///
    /// * `log_origin` - Where the log came from
    /// * `log_type` - The urgency of the log (i.e. warning, error)
    /// * `description` - A string containing a description of why the log is being made
    ///
    /// # Examples
    ///
    /// ```
    /// let log = Log::new(LogOrigin::Main, LogType::Normal, "This is a test log!".to_string());
    /// ```
    pub fn new(log_origin: LogOrigin, log_type: LogType, description: String) -> Self {
        let time: String = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(duration) => duration.as_secs().to_string(),
            Err(_) => "Unknown time".to_string(),
        };

        Self {
            time,
            log_origin,
            log_type,
            description,
        }
    }
}

impl ToString for Log {
    fn to_string(&self) -> String {
        let log_origin_string = match &self.log_origin {
            LogOrigin::MainThread => "[MainThread]".to_string(),
            LogOrigin::MatrixControlThread => "[MatrixControlThread]".to_string(),
            LogOrigin::LoggingThread => "[LoggingThread]".to_string(),
            LogOrigin::Plugin(id) => {
                format!("[Plugin({id})]")
            }
        };

        let log_type_string = match &self.log_type {
            LogType::Normal => "",
            LogType::Warning => "WARN: ",
            LogType::Error => "ERROR: ",
        };

        let description_string = self.description.clone();

        let time_string = self.time.clone();

        format!("{time_string} | {log_origin_string} {log_type_string}{description_string}")
    }
}
