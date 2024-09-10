use std::collections::HashMap;

use log::error;
use rusoto_core::RusotoError;
use rusoto_dynamodb::{
    AttributeValue, DynamoDb, DynamoDbClient, GetItemError, GetItemInput, UpdateItemError,
    UpdateItemInput,
};
use thiserror::Error;

use crate::Deal;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("DynamoDB get error: {0}")]
    Get(#[from] RusotoError<GetItemError>),

    #[error("DynamoDB update error: {0}")]
    Update(#[from] RusotoError<UpdateItemError>),

    #[error("Deserialization error: {0}")]
    Deserialization(#[from] serde_dynamodb::Error),
}

/// Handles interaction with DynamoDB
pub struct DynamoRepository {
    client: DynamoDbClient,
    table_name: String,
}

impl DynamoRepository {
    #[must_use]
    pub fn new(client: DynamoDbClient, table_name: &str) -> Self {
        DynamoRepository {
            client,
            table_name: table_name.to_string(),
        }
    }

    /// Fetches a deal from DynamoDB
    pub async fn get_deal(&self, deal_id: &str) -> Result<Option<Deal>, RepositoryError> {
        let params = GetItemInput {
            table_name: self.table_name.clone(),
            key: {
                let mut key = HashMap::new();
                key.insert(
                    "id".to_string(),
                    AttributeValue {
                        s: Some(deal_id.to_string()),
                        ..Default::default()
                    },
                );
                key
            },
            ..Default::default()
        };

        match self.client.get_item(params).await {
            Ok(result) => {
                if let Some(item) = result.item {
                    let deal: Deal = serde_dynamodb::from_hashmap(item)?;
                    Ok(Some(deal))
                } else {
                    Ok(None)
                }
            }
            Err(err) => {
                error!("Failed to get deal: {:?}", err);
                Err(RepositoryError::Get(err))
            }
        }
    }

    /// Updates a deal in DynamoDB
    pub async fn update_deal(&self, deal: &Deal) -> Result<(), RepositoryError> {
        let mut expression_attribute_values = HashMap::new();
        expression_attribute_values.insert(
            ":v".to_string(),
            AttributeValue {
                n: Some(deal.votes.to_string()),
                ..Default::default()
            },
        );
        expression_attribute_values.insert(
            ":d".to_string(),
            AttributeValue {
                s: deal.date.clone(),
                ..Default::default()
            },
        );
        expression_attribute_values.insert(
            ":t".to_string(),
            AttributeValue {
                s: Some(deal.title.clone()),
                ..Default::default()
            },
        );

        let params = UpdateItemInput {
            table_name: self.table_name.clone(),
            key: {
                let mut key = HashMap::new();
                key.insert("id".to_string(), AttributeValue {
                    s: Some(deal.id.clone()),
                    ..Default::default()
                });
                key
            },
            expression_attribute_values: Some(expression_attribute_values),
            update_expression: Some("SET created = if_not_exists(created, :d), votes = :v, title = if_not_exists(title, :t)".to_string()),
            ..Default::default()
        };

        match self.client.update_item(params).await {
            Ok(_) => Ok(()),
            Err(err) => {
                error!("Failed to update deal: {:?}", err);
                Err(RepositoryError::Update(err))
            }
        }
    }
}
