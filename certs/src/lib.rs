use std::process::{Command,Stdio};

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