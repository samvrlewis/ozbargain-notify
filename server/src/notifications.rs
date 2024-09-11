use fcm::{
    message::{AndroidConfig, AndroidNotification, Message, Notification, Target},
    FcmClient, FcmClientError,
};
use log::info;
use serde_json::json;
use thiserror::Error;

use crate::Deal;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Push error: {0}")]
    PushError(#[from] FcmClientError),
}

/// Manages notifications
pub struct NotificationService {
    client: FcmClient,
}

impl NotificationService {
    pub async fn new() -> Result<Self, Error> {
        let client = fcm::FcmClient::builder()
            // Comment to use GOOGLE_APPLICATION_CREDENTIALS environment
            // variable. The variable can also be defined in .env file.
            .service_account_key_json_path("service_account_key.json")
            .build()
            .await?;

        Ok(Self { client })
    }

    pub async fn notify(&self, deal: &Deal, category: &str) -> Result<(), Error> {
        info!(
            "Sending notification for deal: {} to category: {}",
            deal.title, category
        );

        let message = Message {
            data: Some(json!({
               "message": deal.title,
                "notId": deal.id,
                "url_intent": format!("https://www.ozbargain.com.au/node/{}", deal.id),
            })),
            notification: None,
            android: None,
            webpush: None,
            apns: None,
            fcm_options: None,
            target: Target::Topic(category.to_string()),
        };
        let response = self.client.send(message).await?;

        info!("Sent notification with response {:?}", response);

        Ok(())
    }
}
