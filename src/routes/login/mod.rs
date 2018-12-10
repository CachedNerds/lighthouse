use crate::db;
use crate::state::AppState;
use actix_web::error::Error;
use actix_web::*;
use actix_web::{AsyncResponder, FutureResponse, HttpRequest, HttpResponse, Json};
use futures::Future;

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
    Ok(Json(Response { flows: FLOWS }))
}

pub fn login(req: &HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let username = "username".to_string();
    req.state()
        .db
        .send(db::messages::GetUser { username })
        .from_err()
        .and_then(|result| match result {
            Ok(user) => Ok(HttpResponse::Ok().json(user)),
            Err(_) => Ok(HttpResponse::Ok().json("{}".to_string())),
        })
        .responder()
}
