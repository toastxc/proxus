pub mod data;

pub mod tcp;
pub mod threads;
pub type Result<T> = core::result::Result<T, anyhow::Error>;
