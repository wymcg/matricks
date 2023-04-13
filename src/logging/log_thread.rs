use crate::logging::log::Log;
use std::fs::File;
use std::io::Write;
use std::sync::mpsc;
use std::thread::JoinHandle;
use std::time::SystemTime;
use std::{fs, thread};

/// Thread to handle logging of Matricks events
pub struct LoggingThread {
    log_path: String,
}

impl LoggingThread {
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

    /// Start the logging thread. Returns the join handle and the log sender
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

    fn log(log_path: String, rx: mpsc::Receiver<Log>) {
        // create the file
        let mut file = File::create(log_path).expect("Failed to create log file!");

        // write each log event to the file as the thread receives them
        for log in rx {
            writeln!(&mut file, "{}", log.to_string()).expect("Failed to write log to logfile!");
        }
    }
}
