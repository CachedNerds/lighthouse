use crate::db::models;
use actix_web::actix::Message;

pub struct GetUser {
    pub username: String,
}

impl Message for GetUser {
    type Result = Result<Option<models::User>, actix_web::Error>;
}
