use ubook::api::AuthAPI;
use ubook::backend::BoluobaoHost;
use ubook::share::*;

use std::io::{self, Write};

fn main() -> ubook::Result<()> {
    let mut account = String::new();
    let mut password = String::new();

    let stdin = io::stdin();

    print!("ACCOUNT : ");
    io::stdout().flush()?;
    stdin.read_line(&mut account)?;

    print!("PASSWORD: ");
    io::stdout().flush()?;
    stdin.read_line(&mut password)?;

    let host = BoluobaoHost {};
    match host.try_login(&account.trim(), &password.trim()) {
        Err(msg) => println!("{} [ERROR] {}", Timestamp::now(), msg),
        Ok(user_id) => println!("{} [INFO] login with user_id={}", Timestamp::now(), user_id),
    }

    Ok(())
}
