use base64::{Engine as _, engine::general_purpose};

use crate::{
    config::Config,
    error::{AppError, Result},
    models::{response::NowPlayingResponse, spotify::*},
};

pub struct SpotifyService {
    client: reqwest::Client,
    config: Config,
}

const REFRESHURL: &str = "https://accounts.spotify.com/api/token";
const APIURL: &str = "https://api.spotify.com/v1/me/player/currently-playing";
const AUTH: &str = "Authorization";

impl SpotifyService {
    pub fn new(config: Config) -> Self {
        Self {
            client: reqwest::Client::new(),
            config,
        }
    }

    async fn get_access_token(&self) -> Result<String> {
        let auth = general_purpose::STANDARD.encode(format!(
            "{}:{}",
            self.config.spotify_client_id, self.config.spotify_client_secret
        ));

        let params = [
            ("grant_type", "refresh_token"),
            ("refresh_token", &self.config.spotify_refresh_token),
        ];

        let response = self
            .client
            .post(REFRESHURL)
            .header(AUTH, format!("Basic {}", auth))
            .form(&params)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(AppError::SpotifyApi(
                "Failed to get access token".to_string(),
            ));
        }

        let token_response: TokenResponse = response.json().await?;
        Ok(token_response.access_token)
    }

    pub async fn get_now_playing(&self) -> Result<NowPlayingResponse> {
        let access_token = self.get_access_token().await?;

        let response = self
            .client
            .get(APIURL)
            .header(AUTH, format!("Bearer {}", access_token))
            .send()
            .await?;

        if response.status() == 204 {
            return Ok(NowPlayingResponse::not_playing());
        }

        if !response.status().is_success() {
            return Err(AppError::SpotifyApi(
                "Failed to get currently playing track".to_string(),
            ));
        }

        let playing: CurrentlyPlaying = response.json().await?;

        if !playing.is_playing || playing.item.is_none() {
            return Ok(NowPlayingResponse::not_playing());
        }

        let track = playing.item.unwrap();
        let artists = track
            .artists
            .iter()
            .map(|a| a.name.as_str())
            .collect::<Vec<_>>()
            .join(",");

        Ok(NowPlayingResponse {
            is_playing: true,
            title: Some(track.name),
            artist: Some(artists),
            album: Some(track.album.name),
            album_image_url: track.album.images.first().map(|image| image.url.clone()),
            song_url: Some(track.external_urls.spotify),
        })
    }
}
