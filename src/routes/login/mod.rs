use actix_web::error::Error;
use actix_web::{HttpRequest, Json};
use crate::state::AppState;

const DUMMY: &str = "m.login.dummy";

type Flows = [LoginFlow; 1];
const FLOWS: Flows = [LoginFlow { login_type: DUMMY }];

#[derive(Serialize)]
pub struct Response {
    flows: Flows,
}

#[derive(Serialize)]
pub struct LoginFlow {
    #[serde(rename = "type")]
    login_type: &'static str,
}

pub fn supported_login_types(_: &HttpRequest<AppState>) -> Result<Json<Response>, Error> {
    Ok(Json(Response {
        flows: FLOWS,
    }))
}
