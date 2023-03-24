use std::error::Error;
use chrono::{DateTime, Utc};
use reqwest::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = Url::parse("https://groupecreditagricole.jobs")?;
    let domain = url.host_str().ok_or("invalid URL")?;

    let resp = reqwest::get(url.as_str()).await?;
    let cert = resp
        .tls_certificate()
        .ok_or("no TLS certificate")?
        .as_ref();

    let cert_expiry = cert.not_after();
    let today = Utc::now().date();
    let days_until_expiry = cert_expiry
        .signed_duration_since(today.and_hms(0, 0, 0))
        .num_days();

    println!(
        "Le certificat du site {} expire dans {} jours",
        domain, days_until_expiry
    );

    Ok(())
}
