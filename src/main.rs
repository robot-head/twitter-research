use anyhow::Error;
use pico_args::Arguments;
use std::io::{stdin, Read};
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = parse_args()?;
    let con_token = egg_mode::KeyPair::new(args.consumer_key, args.consumer_secret);
    // "oob" is needed for PIN-based auth; see docs for `request_token` for more info
    let request_token = egg_mode::auth::request_token(&con_token, "oob").await?;
    let auth_url = egg_mode::auth::authorize_url(&request_token);

    println!("Please visit this URL to authorize twitter research:");
    println!("{}", auth_url);
    println!("-----------");
    println!("Now please enter the pin code from Twitter: ");
    // give auth_url to the user, they can sign in to Twitter and accept your app's permissions.
    // they'll receive a PIN in return, they need to give this to your application
    let mut verifier = String::new();
    stdin().read_line(&mut verifier)?;

    // note this consumes con_token; if you want to sign in multiple accounts, clone it here
    let (token, user_id, screen_name) =
        egg_mode::auth::access_token(con_token, &request_token, verifier).await?;

    println!(
        "Authenticated as {}, userId: {}, token: {:#?}",
        screen_name, user_id, token
    );
    // token can be given to any egg_mode method that asks for a token
    // user_id and screen_name refer to the user who signed in
    Ok(())
}

struct Args {
    consumer_key: String,
    consumer_secret: String,
}

fn parse_args() -> Result<Args, Error> {
    let mut args = Arguments::from_env();
    Ok(Args {
        consumer_key: args.value_from_str("--consumer_key")?,
        consumer_secret: args.value_from_str("--consumer_secret")?,
    })
}
