use anyhow::Error;
use std::{fs::File, io, io::BufRead};

pub type Accounts = Vec<String>;

pub fn read_accounts(path: String) -> Result<Accounts, Error> {
    let file = File::open(path)?;
    let lines = io::BufReader::new(file).lines();
    let accounts = lines
        .filter_map(|line| {
            if let Ok(account_name) = line {
                Some(account_name)
            } else {
                None
            }
        })
        .collect();
    Ok(accounts)
}
