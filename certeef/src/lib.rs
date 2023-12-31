use std::process::{Command,Stdio};
use std::str;
use chrono::NaiveDateTime;
use std::fs;
use serde_json::{json,Value};

// This is the main library for certeef
pub fn check_expiration_date_of(url: &str) -> u32 {
    let openssl_output_first = Command::new("openssl")
                            .arg("s_client")
                            .arg("-connect")
                            .arg(format!("{}:443",url))
                            .arg("-servername")
                            .arg(url)
                            .stdout(Stdio::piped())
                            .spawn()
                            .unwrap();

    let openssl_output_second = Command::new("openssl")
                            .arg("x509")
                            .arg("-noout")
                            .arg("-dates")
                            .stdin(Stdio::from(openssl_output_first.stdout.unwrap())) // Pipe through.
                            .stdout(Stdio::piped())
                            .spawn()
                            .unwrap();
    let output = openssl_output_second.wait_with_output().unwrap();
    let openssl_stdout = str::from_utf8(&output.stdout).unwrap();
    let criteria = "notAfter=";
    let not_after_index = openssl_stdout.find(criteria).expect("failed to find notAfter=");

    let not_after = &openssl_stdout[not_after_index+criteria.len()..];
    let end_index = not_after.find("\n").expect("failed to find end of notAfter");
    let not_after = &not_after[..end_index];

    return calculate_days_until_expiry(not_after);

}
// 
fn calculate_days_until_expiry(not_after: &str) -> u32 {

    let format_str = "%b %d %H:%M:%S %Y %Z";

    let parsed_date = NaiveDateTime::parse_from_str(not_after, format_str);
    match parsed_date {
        Ok(date_time) => println!("Parsed date: {:?}", date_time),
        Err(e) => println!("Error: {:?}", e),
    }

    let now = chrono::Utc::now();
    let expiry_duration = parsed_date.unwrap().timestamp() - now.timestamp();
    let days_until_expiry = (expiry_duration / (24 * 60 * 60)) as u32;
    
    return days_until_expiry;
}

pub fn generate_self_signed_certificate() -> std::io::Result<Value> {
    let get_self_signed_cert = Command::new("openssl")
        .args(&[
            "req",
            "-x509",
            "-newkey",
            "rsa:4096",
            "-keyout",
            "key.pem",
            "-out",
            "cert.pem",
            "-days",
            "365",
            "-nodes",
            "-subj",
            "/CN=localhost",
        ]).stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let output = get_self_signed_cert.wait_with_output().unwrap();

    if output.status.success() {
        let cert = fs::read_to_string("cert.pem")?;
        let key = fs::read_to_string("key.pem")?;
        let data = json!({
            "certificate": cert,
            "key": key,
        });
        println!("certificate: {}", data["certificate"]);
        Ok(data)
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::Other, String::from_utf8_lossy(&output.stderr).to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_expiration_date_of() {
        let url = "www.github.com";
        let result = check_expiration_date_of(url);
        assert!(result > 0);
    }   

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use serde_json::Value;

    #[test]
    fn test_good_date_calculate_days_until_expiry() {
        assert_eq!(calculate_days_until_expiry("Jan 01 00:00:01 2099 GMT"), 27419);
    }

    #[test]
    fn test_bad_date_calculate_days_until_expiry() {
        assert_ne!(calculate_days_until_expiry("Jul 26 23:59:59 2024 GMT"), 40);
    }

    #[test]
    fn test_generate_self_signed_certificate() {
        let result = generate_self_signed_certificate();

        match result {
            Ok(json_string) => {
                let parsed: Value = json_string;
                let certificate = parsed["certificate"].as_str().unwrap();
                let key = parsed["key"].as_str().unwrap();
                assert!(certificate.contains("BEGIN CERTIFICATE"));
                assert!(key.contains("BEGIN PRIVATE KEY"));

                println!("Certificate: {}", certificate);
                println!("Key: {}", key);

            }
            Err(e) => eprintln!("Failed to generate certificate: {}", e),
        }
    }
}

}