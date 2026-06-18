/// AOT compilation error.
#[derive(Debug)]
pub enum AotError {
    CompilationError(String),
    WasmtimeError(String),
    IOError(String),
    ConfigError(String),
    RuntimeError(String),
}

impl std::fmt::Display for AotError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AotError::CompilationError(msg) => write!(f, "AOT compilation error: {}", msg),
            AotError::WasmtimeError(msg) => write!(f, "Wasmtime error: {}", msg),
            AotError::IOError(msg) => write!(f, "IO error: {}", msg),
            AotError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            AotError::RuntimeError(msg) => write!(f, "Runtime error: {}", msg),
        }
    }
}

impl std::error::Error for AotError {}

impl From<std::io::Error> for AotError {
    fn from(error: std::io::Error) -> Self {
        AotError::IOError(error.to_string())
    }
}

impl From<wasmtime::Error> for AotError {
    fn from(error: wasmtime::Error) -> Self {
        AotError::WasmtimeError(error.to_string())
    }
}
