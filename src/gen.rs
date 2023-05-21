use super::spec;
use openssl::asn1::Asn1Integer;
use openssl::asn1::Asn1Time;
use openssl::bn::BigNum;
use openssl::error::ErrorStack;
use openssl::hash::MessageDigest;
use openssl::nid::Nid;
use openssl::pkey::PKey;
use openssl::rand;
use openssl::rsa::Rsa;
use openssl::x509::extension::BasicConstraints;
use openssl::x509::X509Extension;
use openssl::x509::X509Name;
use openssl::x509::extension::ExtendedKeyUsage;
use openssl::x509::{X509Builder, X509NameBuilder};
use std::fs::File;
use std::io::Write;

pub fn generate_cert(
    cert_dir: &str,
    cert_spec: &spec::Certificate,
) -> Result<(), Box<dyn std::error::Error>> {
    log::debug!("Generating RSA key...");
    let rsa = Rsa::generate(cert_spec.key.bits)?;
    let private_key = PKey::from_rsa(rsa)?;

    log::debug!("Configuring cert params");
    let mut x509_builder = X509Builder::new()?;
    x509_builder.set_version(2)?; // X509v3
    x509_builder.set_subject_name(build_subject_name(&cert_spec.subject)?.as_ref())?;
    x509_builder.set_issuer_name(build_subject_name(&cert_spec.subject)?.as_ref())?;
    x509_builder.set_pubkey(&private_key)?;
    x509_builder.set_not_before(Asn1Time::days_from_now(0)?.as_ref())?;
    x509_builder.set_not_after(Asn1Time::days_from_now(cert_spec.validity.days)?.as_ref())?;
    x509_builder.set_serial_number(gen_serial()?.as_ref())?;
    x509_builder.append_extension(build_basic_constraints_ext()?)?;
    x509_builder.append_extension(build_ext_key_usage()?)?;

    log::debug!("Signing the certificate...");
    x509_builder.sign(&private_key, MessageDigest::sha256())?;
    let certificate = x509_builder.build();

    let private_key_file = format!("{}/{}{}", cert_dir, cert_spec.subject.common, ".key");
    log::debug!("Saving private key to {}", private_key_file);
    let private_key_pem = private_key.private_key_to_pem_pkcs8()?;
    let mut private_key_file = File::create(private_key_file)?;
    private_key_file.write_all(&private_key_pem)?;

    let certificate_file = format!("{}/{}{}", cert_dir, cert_spec.subject.common, ".crt");
    log::debug!("Saving certificate to {}", certificate_file);
    let certificate_pem = certificate.to_der()?;
    let mut certificate_file = File::create(certificate_file)?;
    certificate_file.write_all(&certificate_pem)?;
    Ok(())
}

fn build_subject_name(subj: &spec::Subject) -> Result<X509Name, ErrorStack> {
    let mut builder = X509NameBuilder::new()?;
    builder.append_entry_by_nid(Nid::COMMONNAME, &subj.common)?;
    builder.append_entry_by_nid(Nid::COUNTRYNAME, &subj.country)?;
    builder.append_entry_by_nid(Nid::ORGANIZATIONNAME, &subj.org)?;
    if let Some(org_unit) = &subj.org_unit {
        builder.append_entry_by_nid(Nid::ORGANIZATIONALUNITNAME, org_unit)?;
    }
    if let Some(state) = &subj.state {
        builder.append_entry_by_nid(Nid::STATEORPROVINCENAME, state)?;
    }
    if let Some(locality) = &subj.locality {
        builder.append_entry_by_nid(Nid::LOCALITYNAME, locality)?;
    }
    Ok(builder.build())
}

fn build_basic_constraints_ext() -> Result<X509Extension, ErrorStack> {
    BasicConstraints::new().critical().ca().pathlen(0).build()
}

fn build_ext_key_usage() -> Result<X509Extension, ErrorStack> {
    ExtendedKeyUsage::new().server_auth().client_auth().build()
}

fn gen_serial() -> Result<Asn1Integer, ErrorStack> {
    let mut buf = [0u8; 8];
    rand::rand_bytes(&mut buf)?;
    let bn = BigNum::from_slice(&buf)?;
    Ok(Asn1Integer::from_bn(&bn)?)
}
