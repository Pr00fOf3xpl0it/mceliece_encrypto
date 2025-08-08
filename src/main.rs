mod cli;
mod crypto;
mod keys;
mod sign;
mod constants;
mod utils;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Keygen { out }) => keys::generate_key(&out)?,
        Some(Commands::Encrypt { file, pubkey }) => crypto::encrypt_file(&file, &pubkey)?,
        Some(Commands::Decrypt { file, privkey }) => crypto::decrypt_file(&file, &privkey)?,
        Some(Commands::Sign { file, out }) => {
            sign::keygen_sign(&out)?;
            sign::sign_file(&file, &format!("{}.sk", out), &format!("{}.sig", out))?;
        }
        Some(Commands::Verify { file, sig, pubkey }) => {
            sign::verify_file(&file, &sig, &pubkey)?;
        }
        None => {
             
            println!("Cifrado local con McEliece + AES-GCM + Firma post-cuántica\n");
            println!("► Autor: Mauro Carrillo");
            println!("► HackSyndicate 2025");
            println!("\nUsa '--help' para ver los comandos disponibles.\n");
        }
    }

    Ok(())
}






