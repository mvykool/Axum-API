use axum::{extract::State, response::Json};
use std::sync::Arc;

use crate::{
    error::Result, models::response::NowPlayingResponse, services::spotify::SpotifyService,
};

pub async fn now_playing(
    State(service): State<Arc<SpotifyService>>,
) -> Result<Json<NowPlayingResponse>> {
    let response = service.get_now_playing().await?;
    Ok(Json(response))
}
