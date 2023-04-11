/// Enum of all possible origins of logs
pub enum LogOrigin {
    /// From the main thread
    MainThread,

    /// From the matrix control thread
    MatrixControlThread,

    /// From the logging thread
    LoggingThread,

    /// From a plugin with the given identifier
    Plugin(String),

    /// From an unknown origin
    Other,
}
