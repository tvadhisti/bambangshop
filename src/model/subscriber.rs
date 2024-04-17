use rocket::serder::{Deserialize, Serialize};
use rocket::log;
use rocket::serder::json::to_string;
use rocket::tokio;
use bambangshop::REQWEST_CLIENT;
user crate::model::notification::Notification;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Subscriber{
    pub url: String,
    pub name: String,
}