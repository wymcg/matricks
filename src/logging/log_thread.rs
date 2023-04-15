use crate::logging::log::Log;
use crate::logging::log_origin::LogOrigin;
use crate::logging::log_type::LogType;
use std::fs::File;
use std::io::Write;
use std::sync::mpsc;
use std::thread::JoinHandle;
use std::time::SystemTime;
use std::{fs, thread};

/// Thread to handle logging of Matricks events
pub struct LoggingThread {
    /// Path to the log
    log_path: String,
}

impl LoggingThread {
    /// Create a new (inactive) logging thread
    ///
    /// # Arguments
    ///
    /// * `log_dir` - Directory to write new log to
    ///
    pub fn new(log_dir: String) -> Self {
        // create the log directory if it doesn't exist
        fs::create_dir_all(log_dir.clone()).expect("Unable to make log directory!");

        // get unique time portion of the log file name
        let time_uid = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Unable to get duration since epoch!")
            .as_secs();

        // assemble the path for the log
        let log_path = format!("{log_dir}/mtx{time_uid}.log");

        Self { log_path }
    }

    /// Start the logging thread, returning the join handle and a sender for logs
    pub fn start(&mut self) -> (JoinHandle<()>, mpsc::Sender<Log>) {
        // make the channels
        let (tx, rx) = mpsc::channel::<Log>();

        // grab the log path to use
        let log_path = self.log_path.clone();

        // start the thread
        let handle = thread::spawn(|| Self::log(log_path, rx));

        // return the join handle and the sender
        (handle, tx)
    }

    /// Log writing loop, called by LoggingThread::start() to spawn the log thread
    fn log(log_path: String, rx: mpsc::Receiver<Log>) {
        // create the file
        let mut file = File::create(log_path).expect("Failed to create log file!");

        // write an initial log just to make sure everything is working ok
        writeln!(
            &mut file,
            "{}",
            Log::new(
                LogOrigin::LoggingThread,
                LogType::Normal,
                "Successfully started the logging thread!".to_string()
            )
            .to_string()
        )
        .expect("Failed to write log to logfile!");

        // write each log event to the file as the thread receives them
        for log in rx {
            writeln!(&mut file, "{}", log.to_string()).expect("Failed to write log to logfile!");
        }
    }
}
