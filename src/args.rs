use anyhow::Error;
use pico_args::Arguments;

pub struct Args {
    pub consumer_key: String,
    pub consumer_secret: String,
    pub accounts_file: String,
}

pub fn parse_args() -> Result<Args, Error> {
    let mut args = Arguments::from_env();
    Ok(Args {
        consumer_key: args.value_from_str("--consumer_key")?,
        consumer_secret: args.value_from_str("--consumer_secret")?,
        accounts_file: args.value_from_str("--accounts_file")?,
    })
}
