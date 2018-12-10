use crate::db::models;
use actix_web::actix::Message;
use actix_web::Error;

pub struct GetUser {
    pub username: String,
}

impl Message for GetUser {
    type Result = Result<Option<models::User>, actix_web::Error>;
}
