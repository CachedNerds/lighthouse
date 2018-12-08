use actix_web::error::Error;
use actix_web::{HttpRequest, Json};
use crate::state::AppState;

const VERSION_SUPPORTED: &str = "r0.4.0";

#[derive(Serialize)]
pub struct Versions {
    versions: Vec<&'static str>,
}

pub fn versions(_: &HttpRequest<AppState>) -> Result<Json<Versions>, Error> {
    Ok(Json(Versions {
        versions: vec![VERSION_SUPPORTED],
    }))
}
