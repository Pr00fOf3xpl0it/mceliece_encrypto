use anyhow::Result;
use std::fs;

/// Lee todo el contenido de un archivo como bytes
pub fn read_bytes(path: &str) -> Result<Vec<u8>> {
    let data = fs::read(path)?;
    Ok(data)
}

/// Escribe un slice de bytes en un archivo
pub fn write_bytes(path: &str, data: &[u8]) -> Result<(), std::io::Error> {
    fs::write(path, data)
}

