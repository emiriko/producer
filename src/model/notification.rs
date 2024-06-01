use bambangshop::REQWEST_CLIENT;
use rocket::{serde::{json::to_string, Deserialize, Serialize}, tokio};
use super::subscriber::Subscriber;


#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Notification {
    pub product_tile: String,
    pub product_type: String,
    pub product_url: String,
    pub subscriber_name: String,
    pub status: String,
}

impl Subscriber {
    #[tokio::main]
    pub async fn notify(&self, payload: Notification) {
        REQWEST_CLIENT
            .post(&self.url)
            .header("Content-Type", "JSON")
            .body(to_string(&payload).unwrap())
            .send().await.ok();
        warn_!("Sent {} notification of [{}] {}, to: {}", payload.status, payload.product_type, payload.product_tile, self.url);
    }
}