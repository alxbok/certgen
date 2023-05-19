mod certgen;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv()
        .map_err(|err| format!("Failed to init environment from .env file, cause: {}", err))?;
    env_logger::init_from_env(env_logger::Env::new());
    log::debug!("Using OpenSSL {}", openssl_version());
    let cert_dir = "generated-certs";
    let _dir_status = std::fs::create_dir(cert_dir);
    certgen::generate_self_signed_certificate(cert_dir)?;
    Ok(())
}

fn openssl_version() -> String {    
    let version = openssl::version::number();
    let major = (version >> 28) & 0xFF;
    let minor = (version >> 20) & 0xFF;
    let patch = (version >> 12) & 0xFF;
    format!("{}.{}.{}", major, minor, patch)
}
