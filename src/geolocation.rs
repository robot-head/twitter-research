use crate::account_reader::Accounts;
use crate::auth::Connection;
use anyhow::Error;
use chrono::{DateTime, Utc};
use egg_mode::tweet::user_timeline;
use std::collections::HashMap;

pub type Geolocations = HashMap<String, TimedGeolocations>;
pub type TimedGeolocations = Vec<TimedGeolocation>;

#[derive(Debug)]
pub struct TimedGeolocation(DateTime<Utc>, Geolocation);
#[derive(Debug)]
pub struct Geolocation(f64, f64);

pub async fn geolocate(accounts: Accounts, connection: Connection) -> Result<Geolocations, Error> {
    let mut geolocations = HashMap::new();
    for account in accounts {
        let mut timed_geolocations = vec![];
        let timeline = user_timeline(account.clone(), true, false, &connection.token);
        let (_timeline, feed) = timeline.start().await?;
        for tweet in &*feed {
            if let Some((x, y)) = tweet.coordinates {
                timed_geolocations.push(TimedGeolocation(tweet.created_at, Geolocation(x, y)));
            }
        }
        geolocations.insert(account, timed_geolocations);
    }
    Ok(geolocations)
}
