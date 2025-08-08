use std::{fs, thread::Builder};
use anyhow::{anyhow, Context, Result};

// struct y funciones de Classic McEliece
use pqcrypto_classicmceliece::mceliece8192128f::{
    keypair,
    PublicKey as McPublicKey,
    SecretKey as McSecretKey,
    public_key_bytes,
    secret_key_bytes,
};

// traits que añaden `as_bytes()` y `from_bytes()`
use pqcrypto_traits::kem::{PublicKey, SecretKey};

/// Genera un par de claves McEliece y las guarda en archivos.
/// Ejecuta `keypair()` en un hilo con 32 MiB de stack para evitar overflow.
pub fn generate_key(out: &str) -> Result<()> {
    let out = out.to_owned();

    // Arranca hilo con stack aumentado
    let handle = Builder::new()
        .stack_size(32 * 1024 * 1024) // 32 MiB
        .spawn(move || -> Result<(), anyhow::Error> {
            // Genera las claves en este hilo secundario
            let (pk, sk) = keypair();

            let pub_path = format!("{}.pub", out);
            let priv_path = format!("{}.priv", out);

            // .as_bytes() viene del trait PublicKey / SecretKey
            fs::write(&pub_path, pk.as_bytes())
                .context("Error al guardar clave pública")?;
            fs::write(&priv_path, sk.as_bytes())
                .context("Error al guardar clave privada")?;

            println!("[+] Claves generadas:");
            println!("[*] Pública: {} ({} bytes)", pub_path, public_key_bytes());
            println!("[*] Privada: {} ({} bytes)", priv_path, secret_key_bytes());

            Ok(())
        })
        .context("No se pudo arrancar hilo para generación de claves")?;

    // Solo recogemos el `()` o el error, sin copiar estructuras grandes
    handle
        .join()
        .map_err(|_| anyhow!("Pánico en hilo de generación de claves"))??;

    Ok(())
}

/// Carga la clave pública desde un archivo
pub fn load_public_key(path: &str) -> Result<McPublicKey> {
    let data = fs::read(path).context("No se pudo leer la clave pública")?;
    // from_bytes() viene del trait PublicKey
    let pk = McPublicKey::from_bytes(&data).context("Clave pública inválida")?;
    Ok(pk)
}

/// Carga la clave privada desde un archivo
pub fn load_private_key(path: &str) -> Result<McSecretKey> {
    let data = fs::read(path).context("No se pudo leer la clave privada")?;
    let sk = McSecretKey::from_bytes(&data).context("Clave privada inválida")?;
    Ok(sk)
}




