use openssl::asn1::Asn1Integer;
use openssl::asn1::Asn1Time;
use openssl::bn::BigNum;
use openssl::error::ErrorStack;
use openssl::hash::MessageDigest;
use openssl::nid::Nid;
use openssl::pkey::PKey;
use openssl::rsa::Rsa;
use openssl::x509::{X509Builder, X509NameBuilder};
use std::fs::File;
use std::io::Write;

pub fn generate_self_signed_certificate(cert_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Generate a new RSA key pair
    log::debug!("Generating RSA key...");
    let rsa = Rsa::generate(4096)?;
    let private_key = PKey::from_rsa(rsa)?;

    // Create a new certificate
    log::debug!("Configuring cert params");
    let mut x509_builder = X509Builder::new()?;
    x509_builder.set_version(2)?; // X509v3
    x509_builder.set_subject_name(build_subject_name().as_ref())?;
    x509_builder.set_issuer_name(build_subject_name().as_ref())?;
    x509_builder.set_pubkey(&private_key)?;
    x509_builder.set_not_before(Asn1Time::days_from_now(0)?.as_ref())?;
    x509_builder.set_not_after(Asn1Time::days_from_now(365)?.as_ref())?; // Valid for 1 year
    x509_builder.set_serial_number(build_serial(1)?.as_ref())?; // Serial number of the certificate
    x509_builder.append_extension(build_basic_constraints_ext())?;

    // Sign the certificate with the private key
    log::debug!("Signing the certificate...");
    x509_builder.sign(&private_key, MessageDigest::sha256())?;
    let certificate = x509_builder.build();

    // Save the private key to a file
    let private_key_file = format!("{}/{}", cert_dir, "pkey.pem");
    log::debug!("Saving private key to {}", private_key_file);
    let private_key_pem = private_key.private_key_to_pem_pkcs8()?;
    let mut private_key_file = File::create(private_key_file)?;
    private_key_file.write_all(&private_key_pem)?;

    // Save the certificate to a file
    let certificate_file = format!("{}/{}", cert_dir, "cert.pem");
    log::debug!("Saving certificate key to {}", certificate_file);
    let certificate_pem = certificate.to_pem()?;
    let mut certificate_file = File::create(certificate_file)?;
    certificate_file.write_all(&certificate_pem)?;
    log::debug!("Done!");
    Ok(())
}

fn build_subject_name() -> openssl::x509::X509Name {
    let mut builder = X509NameBuilder::new().unwrap();
    builder
        .append_entry_by_nid(Nid::COMMONNAME, "localhost")
        .unwrap();
    builder.build()
}

fn build_basic_constraints_ext() -> openssl::x509::X509Extension {
    let mut extension_builder = openssl::x509::extension::BasicConstraints::new();
    extension_builder.critical().ca().pathlen(0);
    extension_builder.build().unwrap()
}

fn build_serial(num: u32) -> Result<Asn1Integer, ErrorStack> {
    Asn1Integer::from_bn(BigNum::from_u32(num)?.as_ref())
}
