use crate::routes;
use crate::state::AppState;
use actix_web::http::Method;
use actix_web::App;

pub fn create_app(state: AppState) -> App<AppState> {
    App::with_state(state)
        .resource("/_matrix/client/versions", |r| {
            r.method(Method::GET).f(routes::version::versions)
        })
        .resource("/.well-known/matrix/client", |r| {
            r.method(Method::GET).f(routes::discovery::well_known)
        })
        .resource("/_matrix/client/r0/login", |r| {
            r.method(Method::GET)
                .f(routes::login::supported_login_types);
            r.method(Method::POST).f(routes::login::login)
        })
}
