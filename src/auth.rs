use anyhow::Error;
use egg_mode::Token;
use std::io::stdin;

pub struct Connection {
    pub token: Token,
    pub user_id: u64,
    pub screen_name: String,
}

pub async fn authenticate(
    consumer_key: String,
    consumer_secret: String,
) -> Result<Connection, Error> {
    let con_token = egg_mode::KeyPair::new(consumer_key, consumer_secret);
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
    Ok(Connection {
        token,
        user_id,
        screen_name,
    })
}
