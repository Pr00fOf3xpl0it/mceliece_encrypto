use std::{thread::Builder, fs};
use anyhow::{anyhow, bail, Context, Result};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit};
use getrandom::getrandom;
use pqcrypto_classicmceliece::mceliece8192128f::*;
use pqcrypto_traits::kem::{SharedSecret as SharedSecretTrait, Ciphertext as CiphertextTrait};

use crate::constants::{MAGIC_HEADER, NONCE_SIZE};
use crate::utils::{read_bytes, write_bytes};
use crate::keys::{load_public_key, load_private_key};

/// Cifra un archivo usando McEliece (KEM) + AES-GCM, todo en un hilo de 32 MiB.
pub fn encrypt_file(file: &str, pubkey_path: &str) -> Result<()> {
    let file = file.to_owned();
    let pubkey_path = pubkey_path.to_owned();

    // Creamos hilo con stack grande
    let handle = Builder::new()
        .stack_size(32 * 1024 * 1024) // 32 MiB
        .spawn(move || -> Result<(), anyhow::Error> {
            // --- flujo original dentro del hilo ---
            let plaintext = read_bytes(&file)
                .context("No se pudo leer el archivo de entrada")?;
            let pk = load_public_key(&pubkey_path)?;

            let (ss, kem_ct) = encapsulate(&pk);
            let key = Key::<Aes256Gcm>::from_slice(&ss.as_bytes()[..32]);
            let cipher = Aes256Gcm::new(key);

            let mut nonce_bytes = [0u8; NONCE_SIZE];
            getrandom(&mut nonce_bytes)
                .context("Error generando nonce")?;
            let nonce = Nonce::from_slice(&nonce_bytes);

            let ciphertext = cipher
                .encrypt(nonce, plaintext.as_ref())
                .map_err(|e| anyhow!(e))?;

            let kem_bytes = CiphertextTrait::as_bytes(&kem_ct);
            let mut out = Vec::new();
            out.extend_from_slice(MAGIC_HEADER);
            out.extend_from_slice(&(kem_bytes.len() as u32).to_be_bytes());
            out.extend_from_slice(kem_bytes);
            out.extend_from_slice(&nonce_bytes);
            out.extend_from_slice(&ciphertext);

            write_bytes(&format!("{file}.enc"), &out)
                .context("No se pudo escribir el archivo cifrado")?;
            println!("[+] Archivo cifrado: {}.enc", file);
            // --- fin del flujo original ---
            Ok(())
        })
        .context("No se pudo arrancar hilo para cifrado")?;

    // Recogemos sólo el `()` o el error
    handle
        .join()
        .map_err(|_| anyhow!("Pánico en hilo de cifrado"))??
    ;

    Ok(())
}

/// Descifra un archivo cifrado con `encrypt_file`, también en hilo de 32 MiB.
pub fn decrypt_file(file: &str, privkey_path: &str) -> Result<()> {
    let file = file.to_owned();
    let privkey_path = privkey_path.to_owned();

    let handle = Builder::new()
        .stack_size(32 * 1024 * 1024)
        .spawn(move || -> Result<(), anyhow::Error> {
            let data = read_bytes(&file)
                .context("No se pudo leer el archivo cifrado")?;
            if &data[..MAGIC_HEADER.len()] != MAGIC_HEADER {
                bail!("Formato inválido");
            }

            let start = MAGIC_HEADER.len();
            let len_kem = u32::from_be_bytes(data[start..start + 4].try_into().unwrap()) as usize;
            let kem_start = start + 4;
            let kem_bytes = &data[kem_start..kem_start + len_kem];
            let nonce_start = kem_start + len_kem;
            let nonce = &data[nonce_start..nonce_start + NONCE_SIZE];
            let cipher_ct = &data[nonce_start + NONCE_SIZE..];

            let kem_ct_obj = CiphertextTrait::from_bytes(kem_bytes)
                .context("KEM ciphertext inválido")?;
            let sk = load_private_key(&privkey_path)?;
            let ss = decapsulate(&kem_ct_obj, &sk);

            let key = Key::<Aes256Gcm>::from_slice(&ss.as_bytes()[..32]);
            let cipher = Aes256Gcm::new(key);

            let plaintext = cipher
                .decrypt(Nonce::from_slice(nonce), cipher_ct.as_ref())
                .map_err(|e| anyhow!(e))?;

            write_bytes(&format!("{file}_decrypted"), &plaintext)
                .context("No se pudo escribir el archivo descifrado")?;
            println!("[+] Archivo descifrado: {}_decrypted", file);

            Ok(())
        })
        .context("No se pudo arrancar hilo para descifrado")?;

    handle
        .join()
        .map_err(|_| anyhow!("Pánico en hilo de descifrado"))??
    ;

    Ok(())
}



