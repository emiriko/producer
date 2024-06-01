use bambangshop::REQWEST_CLIENT;
use rocket::{serde::{json::to_string, Deserialize, Serialize}, tokio, log};

use super::notification::Notification;


#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Subscriber {
    pub url: String,
    pub name: String,
}

impl Subscriber {
    #[tokio::main]
    pub async fn notify(&self, payload: Notification) {
        REQWEST_CLIENT
            .post(&self.url)
            .header("Content-Type", "JSON")
            .body(to_string(&payload).unwrap())
            .send().await.ok();
        log::warn_!("Sent {} notification of [{}] {}, to: {}", payload.status, payload.product_type, payload.product_tile, self.url);
    }
}