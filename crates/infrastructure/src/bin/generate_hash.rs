// crates/infrastructure/src/bin/generate_hash.rs
// Utilidad para generar hash Argon2id de contraseñas
// Uso: cargo run --bin generate_hash -- <contraseña>

use infrastructure::crypto::hash_password;
use secrecy::SecretString;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Uso: cargo run --bin generate_hash -- <contraseña>");
        eprintln!("Ejemplo: cargo run --bin generate_hash -- Admin123!");
        std::process::exit(1);
    }

    let password = &args[1];
    let secret = SecretString::new(password.to_string().into());
    
    match hash_password(&secret) {
        Ok(hash) => {
            println!("{}", hash);
        }
        Err(e) => {
            eprintln!("Error al generar hash: {}", e);
            std::process::exit(1);
        }
    }
}
