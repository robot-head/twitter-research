use anyhow::Error;

mod args;
mod auth;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = args::parse_args()?;
    let connection = auth::authenticate(args.consumer_key, args.consumer_secret).await?;

    // token can be given to any egg_mode method that asks for a token
    // user_id and screen_name refer to the user who signed in
    Ok(())
}
