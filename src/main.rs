mod certgen;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().map_err(|err| format!("Failed to init environment from .env file, cause: {}", err.to_string()))?;
    env_logger::init_from_env(env_logger::Env::new());
    certgen::generate_self_signed_certificate()?;
    Ok(())
}
