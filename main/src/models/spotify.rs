use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TokenResponse {
    pub access_token: String,
}

#[derive(Deserialize, Debug)]
pub struct CurrentlyPlaying {
    pub is_playing: bool,
    pub item: Option<Track>,
}

#[derive(Deserialize, Debug)]
pub struct Track {
    pub name: String,
    pub artists: Vec<Artist>,
    pub album: Album,
    pub external_urls: ExternalUrls,
}

#[derive(Deserialize, Debug)]
pub struct Artist {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct Album {
    pub name: String,
    pub images: Vec<Image>,
}

#[derive(Deserialize, Debug)]
pub struct Image {
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct ExternalUrls {
    pub spotify: String,
}
