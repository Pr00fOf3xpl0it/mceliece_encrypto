use std::fs;
use anyhow::{Context, Result};

// Traemos todo lo necesario de pqcrypto-dilithium
use pqcrypto_dilithium::dilithium2::{
    keypair,
    PublicKey,
    SecretKey,
    DetachedSignature,
    detached_sign,
    verify_detached_signature,
    public_key_bytes,
    secret_key_bytes,
};

// Importamos los traits que añaden `as_bytes()` y `from_bytes()` a esos tipos
use pqcrypto_traits::sign::{
    PublicKey as PKTrait,
    SecretKey as SKTrait,
    DetachedSignature as SigTrait,
};

/// Genera par de claves Dilithium2 y las guarda en `<out>.pk` y `<out>.sk`
pub fn keygen_sign(out: &str) -> Result<()> {
    let (pk, sk) = keypair();

    let pk_path = format!("{out}.pk");
    let sk_path = format!("{out}.sk");

    // .as_bytes() proviene del trait PKTrait / SKTrait
    fs::write(&pk_path, pk.as_bytes())
        .context("Error al guardar clave pública de firma")?;
    fs::write(&sk_path, sk.as_bytes())
        .context("Error al guardar clave privada de firma")?;

    println!("[+] Claves de firma generadas:");
    println!("[*] Pública: {} ({} bytes)", pk_path, public_key_bytes());
    println!("[*] Privada: {} ({} bytes)", sk_path, secret_key_bytes());
    Ok(())
}

/// Firma un archivo y escribe la firma en `out_sig`
pub fn sign_file(file: &str, sk_path: &str, out_sig: &str) -> Result<()> {
    let msg = fs::read(file)
        .context("No se pudo leer el archivo a firmar")?;
    let sk_bytes = fs::read(sk_path)
        .context("No se pudo leer la clave privada de firma")?;
    let sk = SecretKey::from_bytes(&sk_bytes)
        .context("SecretKey inválida")?;

    // DetachedSignature es el tipo resultante
    let signature: DetachedSignature = detached_sign(&msg, &sk);
    fs::write(out_sig, signature.as_bytes())
        .context("Error al escribir la firma")?;

    println!("[+] Archivo firmado: {}", out_sig);
    Ok(())
}

/// Verifica la firma de `file` con `sig_path` usando la clave pública `pk_path`
pub fn verify_file(file: &str, sig_path: &str, pk_path: &str) -> Result<()> {
    let msg = fs::read(file)
        .context("No se pudo leer el archivo a verificar")?;
    let sig_bytes = fs::read(sig_path)
        .context("No se pudo leer la firma")?;
    let signature = DetachedSignature::from_bytes(&sig_bytes)
        .context("DetachedSignature inválida")?;

    let pk_bytes = fs::read(pk_path)
        .context("No se pudo leer la clave pública de firma")?;
    let pk = PublicKey::from_bytes(&pk_bytes)
        .context("PublicKey inválida")?;

    // Atención al orden: primero &signature, luego &msg, luego &pk
    verify_detached_signature(&signature, &msg, &pk)
        .context("La firma no coincide")?;
    println!("Firma válida");
    Ok(())
}


