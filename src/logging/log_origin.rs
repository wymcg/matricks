/// Origin of a log
pub enum LogOrigin {
    /// From the main thread
    MainThread,

    /// From the matrix control thread
    MatrixControlThread,

    /// From the logging thread
    LoggingThread,

    /// From a plugin with the given identifier
    Plugin(String),
}
