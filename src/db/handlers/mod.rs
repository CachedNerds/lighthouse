use crate::{db::messages, db::models, db::DbExecutor};
use actix_web::actix::Handler;
use actix_web::error;
use diesel;
use diesel::prelude::*;

impl Handler<messages::GetUser> for DbExecutor {
    type Result = Result<Option<models::User>, actix_web::Error>;
    fn handle(&mut self, msg: messages::GetUser, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;
        let conn = &self
            .0
            .get()
            .map_err(|_| error::ErrorInternalServerError("Failed to connect to database"))?;

        let mut items = users
            .filter(username.eq(msg.username.clone()))
            .load::<models::User>(conn)
            .map_err(|_| error::ErrorInternalServerError("Error loading person"))?;
        Ok(items.pop())
    }
}
