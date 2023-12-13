mod audio;
mod matrix;
mod text;
mod encryption;

use clap::{Parser, Subcommand};
use encryption::{gen_key, store_key, load_key, SolutionMethod};
use text::{decrypt_text_with_key, encrypt_text_with_key};

use crate::audio::{decrypt_audio_with_key, encrypt_audio_with_key};


#[derive(Debug, Parser)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generate an encryption key.
    GenKey {
        /// The path of the key file.
        #[arg(short, long)]
        key_path: String,
        
        /// The size of the key matrix (larger is better for security but slower).
        #[arg(short, long)]
        chunk_size: usize,
        
        /// only whole number entries.
        #[arg(short, long)]
        integer: bool,
    },
    /// Encrypte a (txt/wav) file with the specified key.
    Encrypt {
        /// The path of the key file.
        #[arg(short, long)]
        key_path: String,
        
        /// The path to the file to be encrypted.
        #[arg(short, long)]
        file_path: String,
    },
    /// Decrypt a (txt/wav) file with the specified key via direct method.
    DecryptDirect {
        /// The path of the key file.
        #[arg(short, long)]
        key_path: String,
        
        /// The path to the file to be decrypted.
        #[arg(short, long)]
        file_path: String
    },
    /// Decrypt a (txt/wav) file with the specified key via iterative method.
    DecryptIterative {
        /// The path of the key file.
        #[arg(short, long)]
        key_path: String,
        
        /// The path to the file to be decrypted.
        #[arg(short, long)]
        file_path: String,
        
        /// Number of iteratioins for the iterative method.
        #[arg(short, long)]
        iterations: Option<usize>,
    },
}

fn main() {
    let Args { cmd } = Args::parse();

    match cmd {
        Commands::GenKey {
            key_path,
            chunk_size,
            integer
        } => {
            let key = gen_key(chunk_size, integer);
            if chunk_size <= 30 {
                println!("{}", key);
                println!("\n{}", key.tril().dot(&key.triu()).round((chunk_size.ilog10() as usize + 2) * 2));
            }
            store_key(key_path.as_str(), &key).unwrap();
        }
        Commands::Encrypt {
            key_path,
            file_path,
        } => {
            let (_, key) = load_key(key_path.as_str()).unwrap();

            if file_path.ends_with(".txt") {
                encrypt_text_with_key(file_path.as_str(), &key).unwrap();
            } else if file_path.ends_with(".wav") {
                encrypt_audio_with_key(file_path.as_str(), &key).unwrap();
            } else {
                panic!("Invalid file type!");
            }
        },
        Commands::DecryptDirect {
            key_path,
            file_path
        } => {
            let (_, key) = load_key(key_path.as_str()).unwrap();

            if file_path.ends_with(".txt") {
                decrypt_text_with_key(file_path.as_str(), &key, SolutionMethod::Direct).unwrap();
            } else if file_path.ends_with(".wav") {
                decrypt_audio_with_key(file_path.as_str(), &key, SolutionMethod::Direct).unwrap();
            } else {
                panic!("Invalid file type!");
            }
        },
        Commands::DecryptIterative {
            key_path,
            file_path,
            iterations,
        } => {
            let (_, key) = load_key(key_path.as_str()).unwrap();

            if file_path.ends_with(".txt") {
                decrypt_text_with_key(file_path.as_str(), &key, SolutionMethod::Iterative(iterations.unwrap_or(100))).unwrap();
            } else if file_path.ends_with(".wav") {
                decrypt_audio_with_key(file_path.as_str(), &key, SolutionMethod::Iterative(iterations.unwrap_or(100))).unwrap();
            } else {
                panic!("Invalid file type!");
            }
        }
    }
}
