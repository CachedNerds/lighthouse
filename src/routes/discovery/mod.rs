use actix_web::error::Error;
use actix_web::{HttpRequest, Json};
use crate::state::AppState;

#[derive(Serialize)]
pub struct HomeserverData {
    base_url: String,
}

#[derive(Serialize)]
pub struct WellKnownData {
    #[serde(rename = "m.homeserver")]
    m_homeserver: HomeserverData,
}

pub fn well_known(req: &HttpRequest<AppState>) -> Result<Json<WellKnownData>, Error> {
    let domain = req.state().domain.clone();

    Ok(Json(WellKnownData {
        m_homeserver: HomeserverData { base_url: domain },
    }))
}
