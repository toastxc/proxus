pub mod data;

pub mod tcp;
pub type Result<T> = core::result::Result<T, anyhow::Error>;
