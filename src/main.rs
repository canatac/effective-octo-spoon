use std::env;
use ansi_term::Colour;
use certeef::check_expiration_date_of;

fn main() {
    
    let args: Vec<String> = env::args().collect();
    let url = &args[2];

    println!("{} expires in {} days", Colour::Yellow.bold().paint(url), Colour::Yellow.bold().paint(check_expiration_date_of(url).to_string()));
   
}

