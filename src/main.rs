mod certgen;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv()
        .map_err(|err| format!("Failed to init environment from .env file, cause: {}", err))?;
    env_logger::init_from_env(env_logger::Env::new());
    log::debug!("Using {}", openssl::version::version());
    let cert_dir = "generated-certs";
    let _dir_status = std::fs::create_dir(cert_dir);
    certgen::generate_self_signed_certificate(cert_dir)?;
    Ok(())
}
