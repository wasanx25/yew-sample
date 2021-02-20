use serde_derive::Deserialize;

#[derive(Clone, Deserialize, Debug, PartialEq, Eq)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub username: String,
    pub email: String,
    pub website: String,
}
