/// Reason for log
pub enum LogType {
    /// Log was made as part of normal operation
    Normal,

    /// Log was made because of a non-catastrophic error
    Warning,

    /// Log was made because of a catastrophic error
    Error,
}
