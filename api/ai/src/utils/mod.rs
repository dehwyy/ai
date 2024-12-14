pub mod types;

pub type AnyError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type AnyResult<T = ()> = Result<T, AnyError>;
