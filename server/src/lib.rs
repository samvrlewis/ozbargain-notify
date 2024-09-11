pub mod deal_service;
pub mod notifications;
pub mod ozbargain;
pub mod persist;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Deal {
    pub id: String,
    pub title: String,
    pub votes: u32,
    pub date: Option<String>,
}
