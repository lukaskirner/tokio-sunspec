#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("client error")]
    Client(),

    #[error("unsupported model (id: {0})")]
    UnsupportedModel(u16),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}
