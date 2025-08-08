use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "mceliece_hacksyndicate",
    version = "1.0",
    author = "Mauro Carrillo - HackSyndicate 2025",
    about = "Cifrado local con McEliece + AES + Firma post-cuántica",
    long_about = "
Cifrado local con McEliece + AES-GCM + Firma post-cuántica (Dilithium2)

► Autor: Mauro Carrillo
► HackSyndicate 2025
"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Genera un par de claves pública/privada (McEliece)
    Keygen {
        /// Base del nombre de salida (sin extensión)
        #[arg(short, long)]
        out: String,
    },

    /// Cifra un archivo usando la clave pública (.pub)
    Encrypt {
        /// Ruta del archivo a cifrar
        #[arg(short, long)]
        file: String,
        /// Ruta a la clave pública (.pub)
        #[arg(short, long)]
        pubkey: String,
    },

    /// Descifra un archivo cifrado usando la clave privada (.priv)
    Decrypt {
        /// Ruta del archivo cifrado (.enc)
        #[arg(short, long)]
        file: String,
        /// Ruta a la clave privada (.priv)
        #[arg(short, long)]
        privkey: String,
    },

    /// Genera un par de claves (Dilithium2) y firma un archivo
    Sign {
        /// Ruta del archivo a firmar
        #[arg(short, long)]
        file: String,
        /// Base del nombre de salida para clave y firma
        #[arg(short, long)]
        out: String,
    },

    /// Verifica la firma de un archivo usando clave pública (Dilithium2)
    Verify {
        /// Ruta del archivo original
        #[arg(short, long)]
        file: String,
        /// Ruta al archivo de firma (.sig)
        #[arg(short, long)]
        sig: String,
        /// Ruta a la clave pública (.pk)
        #[arg(short, long)]
        pubkey: String,
    },
}


