use crate::{db::messages, db::models, db::DbExecutor};
use actix_web::actix::Handler;
use actix_web::error;
use diesel;
use diesel::prelude::*;
use actix_web::http::StatusCode;

impl Handler<messages::GetUser> for DbExecutor {
    type Result = Result<Option<models::User>, actix_web::Error>;
    fn handle(&mut self, msg: messages::GetUser, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::users;
        use crate::schema::users::dsl::*;
        let conn = &self
            .0
            .get()
            .map_err(|_|  error::InternalError::new("Error connecting to db", StatusCode::from_u16(400).unwrap()))?;
        let mut items = users::table
            .filter(username.eq(msg.username.to_string()))
            .load::<models::User>(conn)
            .map_err(|_| error::InternalError::new("Error loading person", StatusCode::from_u16(400).unwrap()))?;
        Ok(items.pop())
    }
}
