use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::io::Read;
use std::str::FromStr;
use std::time::SystemTime;

use serde::de::{self, Deserializer};

static ONE_MONTH_IN_SECS: u64 = 60 * 60 * 24 * 30;

#[derive(Deserialize, Debug)]
struct Album {
    name: String,
    #[serde(deserialize_with = "from_artist_struct")]
    artist: String,
    #[serde(deserialize_with = "from_str")]
    playcount: u32,
    #[serde(default)]
    image: String,
}

fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(de::Error::custom)
}

fn from_artist_struct<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s = HashMap::<String, String>::deserialize(deserializer)?;
    match s.get("#text") {
        Some(val) => Ok(val.to_string()),
        None => Ok("".to_string()),
    }
}



fn fetch_recent_albums(user_id: &str, api_key: &str, from: u64, to: u64) -> Result<Vec<Album>, Box<Error>> {
    let mut res = reqwest::blocking::get(
        format!(
            "http://ws.audioscrobbler.com/2.0/?method=user.getweeklyalbumchart&user={}&api_key={}&format=json&from={}&to={}",
            user_id,
            api_key,
            from,
            to
        ).as_str())?;

    if res.status().is_success() {
        let body = res.text_with_charset("utf-8")?;
        let pos = body.rfind(']').unwrap();
        let albums_raw = format!("{}", &body[29..(pos + 1)]);
        let albums: Vec<Album> = serde_json::from_str(albums_raw.as_str())?;
        Ok(albums)
    } else {
        let err = res.text()?;
        Err(err.into())
    }
}

fn fetch_album_image(user_id: &str, api_key: &str, artist: &str, album: &str) -> Result<(), Box<Error>> {
    let mut res = reqwest::blocking::get(
        format!(
            "http://ws.audioscrobbler.com/2.0/?method=album.getinfo&user={}&api_key={}&format=json&artist={}&album={}",
            user_id,
            api_key,
            artist,
            album
        ).as_str())?;

    if res.status().is_success() {
        let body = res.text_with_charset("utf-8")?;
    } else {
        println!("Error: error occurred");
    }

    Ok(())
}

fn main() {
    let current_epoch = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let last_month_epoch = current_epoch - ONE_MONTH_IN_SECS;

    match fetch_recent_albums(
        "wilspi",
        "",
        last_month_epoch,
        current_epoch,
    ) {
        Ok(albums) => {
            if albums.len() < 16 {
                println!("ERR: not enough albums to generate the image");
                return ();
            }
            for album in albums.iter() {
                println!("{:?}", album)
                
            }
        },
        Err(e) => {
            println!("ERR: {:?}", e);
        }
    };
}
