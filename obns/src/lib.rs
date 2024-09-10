pub mod deal_service;
pub mod notifications;
pub mod ozbargain;
pub mod persist;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Deal {
    id: String,
    title: String,
    votes: u32,
    date: Option<String>,
}
