use std::thread;

use bambangshop::{compose_error_response, Result};
use rocket::http::Status;

use crate::{model::{notification::Notification, product::Product, subscriber::Subscriber}, repository::subscriber::SubscriberRepository};

pub struct NotificationService;

impl NotificationService {
    pub fn subscribe(product_type: &str, subscriber: Subscriber) -> Result<Subscriber> {
        let product_type_upper: String = product_type.to_uppercase();
        let product_type_str: &str = &product_type_upper.as_str();

        let subscriber_result: Subscriber = SubscriberRepository::add(product_type_str, subscriber);

        return Ok(subscriber_result);
    }

    pub fn unsubscribe(product_type: &str, url: &str) -> Result<Subscriber> {
        let product_type_upper: String = product_type.to_uppercase();
        let product_type_str: &str = &product_type_upper.as_str();

        let subscriber_result = SubscriberRepository::delete(product_type_str, url);

        if subscriber_result.is_none() {
            return Err(compose_error_response(Status::NotFound, String::from("Subscriber not found.")));
        }

        return Ok(subscriber_result.unwrap());
    }

    pub fn notify(&self, product_type: &str, status: &str, product: Product) {
        let mut payload: Notification = Notification {
            product_tile: product.clone().title,
            product_type: product.clone().product_type,
            product_url: product.clone().get_url(),
            status: String::from(status),
            subscriber_name: String::from(""),
        };

        let subscribers: Vec<Subscriber> = SubscriberRepository::list_all(product_type);

        for subscriber in subscribers {
            payload.subscriber_name = subscriber.clone().name;
            let subscriber_clone = subscriber.clone();

            let payload_clone = payload.clone();

            thread::spawn(move || subscriber_clone.notify(payload_clone));
        }
    }
}