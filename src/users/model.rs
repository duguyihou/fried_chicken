// use chrono::*;

// #[derive(Queryable)]
// pub struct User {
//     pub id: i32,
//     pub username: String,
//     pub email: String,
//     pub created_at: NativeDateTime,
// }

#[derive(Debug)]
pub struct UserData {
    pub username: String,
    pub email: String,
    pub password: String,
}
