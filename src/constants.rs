/// Magic header to identify our encrypted files
pub const MAGIC_HEADER: &[u8; 4] = b"MCEL";

/// Size of AES-GCM nonce in bytes
pub const NONCE_SIZE: usize = 12;

