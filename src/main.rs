use anyhow::Error;

mod account_reader;
mod args;
mod auth;
mod geolocation;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = args::parse_args()?;
    let accounts = account_reader::read_accounts(args.accounts_file)?;
    let connection = auth::authenticate(args.consumer_key, args.consumer_secret).await?;
    let geolocations = geolocation::geolocate(accounts, connection).await?;
    println!("Geolocations: {:#?}", geolocations);
    // token can be given to any egg_mode method that asks for a token
    // user_id and screen_name refer to the user who signed in
    Ok(())
}
