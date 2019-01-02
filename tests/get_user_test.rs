use actix::Addr;
use actix::SyncArbiter;
use actix_web::client::ClientResponse;
use actix_web::http::Method;
use actix_web::test::TestServer;
use diesel::connection::Connection;
use diesel::query_dsl::RunQueryDsl;
use diesel_migrations;
use tempdir::TempDir;

use lighthouse::db;
use lighthouse::db::models::User;
use lighthouse::db::ConnectionType;
use lighthouse::db::DbExecutor;
use lighthouse::state::AppState;

#[derive(Clone)]
pub struct TestDbActor {
    pub connection_string: String,
    pub address: Addr<DbExecutor>,
}

pub fn create_test_db(connection_string: String) -> TestDbActor {
    let connection_string = connection_string.clone();
    let manager = db::create_manager_with_connection_string(connection_string.clone());
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let address = SyncArbiter::start(3, move || DbExecutor(pool.clone()));
    TestDbActor {
        connection_string: connection_string.clone(),
        address,
    }
}

#[test]
fn get_user() {
    // create a temporary directory that cleans itself up
    let tmp_db_dir = TempDir::new("db").unwrap();
    let db_path = tmp_db_dir.path().join("test.db");
    let db_path_str = db_path.to_str().unwrap();
    use lighthouse::schema::users::table;
    let conn = ConnectionType::establish(db_path_str).unwrap();
    // use the special migrations crate to run the migrations on our temp db
    let _ = diesel_migrations::run_pending_migrations(&conn);
    // create a user to insert
    let user = User {
        id: 1,
        username: "username".to_string(),
        password: "jdklfjsd".to_string(),
    };
    // insert a user into the database
    diesel::insert_into(table)
        .values(&user)
        .execute(&conn)
        .expect("Error inserting user");
    drop(conn);
    let connection_string = db_path_str.to_string();

    // make a test server
    let mut test_server = TestServer::with_factory(move || {
        let test_db_actor = create_test_db(connection_string.clone());
        let app_state = AppState {
            domain: "some_domain".to_string(),
            db: test_db_actor.address.clone(),
        };
        // use the helper method to get all our routes for the test server
        lighthouse::app::create_app(app_state)
    });

    // send a request for the user
    let request = test_server
        .client(Method::POST, "/_matrix/client/r0/login")
        .header("content-type", "application/json")
        .json("{}")
        .unwrap();

    // get the response
    let response: ClientResponse = test_server.execute(request.send()).unwrap();

    // ASSERTION
    assert!(response.status().is_success())
}
