use log::info;
use thiserror::Error;

use crate::{
    notifications::NotificationService,
    persist::{DynamoRepository, RepositoryError},
    Deal,
};

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to fetch or update deal: {0}")]
    RepositoryError(#[from] RepositoryError),

    #[error("Notification error: {0}")]
    NotificationError(#[from] crate::notifications::Error),
}

pub struct DealService {
    dynamo_repo: DynamoRepository,
    notification_service: NotificationService,
}

fn should_notify(new_deal: &Deal, old_deal: &Option<Deal>, threshold: u32) -> bool {
    match old_deal {
        Some(old) => old.votes < threshold && new_deal.votes >= threshold,
        None => threshold == 0,
    }
}

impl DealService {
    #[must_use]
    pub fn new(dynamo_repo: DynamoRepository, notification_service: NotificationService) -> Self {
        DealService {
            dynamo_repo,
            notification_service,
        }
    }

    pub async fn process_deal(&self, deal: Deal) -> Result<(), Error> {
        let old_deal = self.dynamo_repo.get_deal(&deal.id).await?;

        let thresholds = vec![("all_deals", 0), ("over_50", 50), ("over_100", 100)];
        let mut deal_updated = false;

        for (category, threshold) in thresholds {
            if should_notify(&deal, &old_deal, threshold) {
                info!("Notifying {} for deal: {}", category, deal.title);
                self.notification_service.notify(&deal, category).await?;
                deal_updated = true;
            } else {
                info!("Deal is known {}", deal.title);
            }
        }

        if deal_updated {
            info!("Updating deal {}", deal.title);
            self.dynamo_repo.update_deal(&deal).await?;
        }

        Ok(())
    }
}
