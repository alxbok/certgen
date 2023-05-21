mod gen;
mod spec;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv()
        .map_err(|err| format!("Failed to init environment from .env file, cause: {}", err))?;
    env_logger::init_from_env(env_logger::Env::new());
    log::debug!("Using {}", openssl::version::version());
    let cert_dir = "certs";
    std::fs::create_dir_all(cert_dir)?;
    let spec_file_content = std::fs::read_to_string("certs.yml")?;
    let spec: spec::Spec = serde_yaml::from_str(&spec_file_content)?;
    for cert_spec in spec.certs {
        log::info!("Generating certificate {}", cert_spec.subject.full_name());
        gen::generate_cert(cert_dir, &cert_spec)?;
    }
    Ok(())
}
