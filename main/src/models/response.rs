use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct NowPlayingResponse {
    #[serde(rename = "isPlaying")]
    pub is_playing: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub album: Option<String>,

    #[serde(rename = "albumImageUrl", skip_serializing_if = "Option::is_none")]
    pub album_image_url: Option<String>,

    #[serde(rename = "songUrl", skip_serializing_if = "Option::is_none")]
    pub song_url: Option<String>,
}

impl NowPlayingResponse {
    pub fn not_playing() -> Self {
        Self {
            is_playing: false,
            title: None,
            artist: None,
            album: None,
            album_image_url: None,
            song_url: None,
        }
    }
}
