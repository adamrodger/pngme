mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

/// Generic PNGme error
pub type Error = Box<dyn std::error::Error>;

/// Generic PNGme result
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    todo!();
}
