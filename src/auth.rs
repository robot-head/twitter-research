use anyhow::Error;
use egg_mode::Token;
use std::io::stdin;

pub async fn authenticate(consumer_key: String, consumer_secret: String) -> Result<Token, Error> {
    let con_token = egg_mode::KeyPair::new(consumer_key, consumer_secret);
    let token = egg_mode::auth::bearer_token(&con_token).await?;

    println!("Authenticated token: {:#?}", token);
    Ok(token)
}
