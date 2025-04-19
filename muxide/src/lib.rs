pub mod mkv;

#[derive(Debug, thiserror::Error)]
pub enum MuxideError {
    #[error("Underlying I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Unexpected end of input")]
    UnexpectedEnd,
    #[error("VINT has incorrect length bit")]
    VintInavlidLength,
}
