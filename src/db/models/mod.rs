use crate::schema::users;

#[derive(Serialize, PartialEq, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}
