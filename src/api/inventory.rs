/*!
Inventory functionality of the [Square API](https://developer.squareup.com).
 */

use crate::client::SquareClient;
use crate::api::{Verb, SquareAPI};
use crate::errors::{InventoryChangeBodyBuildError, SquareError};
use crate::response::SquareResponse;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::objects::{CatalogObject, InventoryAdjustment, InventoryChange, InventoryPhysicalCount, InventoryTransfer};
use crate::objects::enums::InventoryChangeType;

impl SquareClient {
    /// Returns an [Inventory](Inventory) object through which you can make calls specifically to
    /// the Inventory endpoint of the [Square API](https://developer.squareup.com).
    /// # Example: Using the inventory endpoint to make a retrieve_count request.
    /// ```rust
    /// use square_rs::{
    ///     response::{SquareResponse, ResponseError},
    ///     client::SquareClient,
    ///     api::inventory::Inventory,
    ///     };
    ///
    /// async {
    ///     let count = SquareClient::new("some_token")
    ///         .inventory()
    ///         .retrieve_count(
    ///             "some_obj_id".to_string(),
    ///             Some("some_loc_id".to_string())
    ///         )
    ///         .await;
    ///     };
    /// ```
    pub fn inventory(&self) -> Inventory {
        Inventory {
            client: &self,
        }
    }
}

/// Allows you to make calls to the [Square API](https://developer.squareup.com) at the Inventory
/// endpoint with all currently implemented methods.
pub struct Inventory<'a> {
    client: &'a SquareClient
}

impl<'a> Inventory<'a> {

    /// Applies adjustments and counts to the provided item quantities.
    /// [Open in API Reference](https://developer.squareup.com/reference/square/inventory/batch-change-inventory)
    pub async fn batch_change(self, body: InventoryChangeBody)
                                -> Result<SquareResponse, SquareError>{
        self.client.request(
            Verb::POST,
            SquareAPI::Inventory("/changes/batch-create".to_string()),
            Some(&body),
            None,
        ).await
    }

    /// Retrieves the current calculated stock count for a given [CatalogObject](CatalogObject) at
    /// a given set of [Location](Location)s.
    /// [Open in API Reference](https://developer.squareup.com/reference/square/inventory/retrieve-inventory-count)
    pub async fn retrieve_count(self, object_id: String, location_id: Option<String>)
        -> Result<SquareResponse, SquareError>{
        let parameters = match location_id {
            Some(location_id) => Some(vec![("location_id".to_string(), location_id)]),
            None => None
        };

        self.client.request(
            Verb::GET,
            SquareAPI::Inventory(format!("/{}", object_id)),
            None::<&CatalogObject>,
            parameters,
        ).await
    }

    /// Returns the [InventoryAdjustment](InventoryAdjustment) object with the provided adjustment_id.
    /// [Open in API Reference](https://developer.squareup.com/reference/square/inventory/retrieve-inventory-adjustment)
    pub async fn retrieve_adjustment(self, adjustment_id: String)
                                -> Result<SquareResponse, SquareError>{
        self.client.request(
            Verb::GET,
            SquareAPI::Inventory(format!("/adjustments/{}", adjustment_id)),
            None::<&CatalogObject>,
            None,
        ).await
    }

    /// Returns the [InventoryTransfer](InventoryTransfer) object with the provided `transfer_id`.
    /// [Open in API Reference](https://developer.squareup.com/reference/square/inventory/retrieve-inventory-transfer)
    pub async fn retrieve_transfer(self, transfer_id: String)
                                -> Result<SquareResponse, SquareError>{
        self.client.request(
            Verb::GET,
            SquareAPI::Inventory(format!("/transfer/{}", transfer_id)),
            None::<&CatalogObject>,
            None,
        ).await
    }

    /// Returns the [InventoryPhysicalCount](InventoryPhysicalCount) object with the provided `physical_count_id`.
    /// [Open in API Reference](https://developer.squareup.com/reference/square/inventory/retrieve-inventory-physical-count)
    pub async fn retrieve_physical_count(self, physical_count_id: String)
                                -> Result<SquareResponse, SquareError>{
        self.client.request(
            Verb::GET,
            SquareAPI::Inventory(format!("/physical-counts/{}", physical_count_id)),
            None::<&CatalogObject>,
            None,
        ).await
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InventoryChangeBody {
    idempotency_key: String,
    changes: Vec<InventoryChange>,
    ignore_unchanged_counts: Option<bool>,
}

#[derive(Default)]
pub struct InventoryChangeBodyBuilder {
    changes: Vec<InventoryChange>,
    ignore_unchanged_counts: Option<bool>,
}

impl InventoryChangeBodyBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn change(mut self, change: InventoryChange) -> Self {
        self.changes.push(change);

        self
    }

    pub fn change_object_builder(self) -> InventoryChangeObjectBuilder {
        InventoryChangeObjectBuilder {
            inventory_change_body_builder: self,
            inventory_change: InventoryChange {
                adjustment: None,
                measurement_unit: None,
                measurement_unit_id: None,
                physical_count: None,
                transfer: None,
                inventory_change_type: InventoryChangeType::PhysicalCount
            }
        }
    }

    pub async fn build(mut self) -> Result<InventoryChangeBody, InventoryChangeBodyBuildError> {
        if self.changes.len() == 0 {
            return Err(InventoryChangeBodyBuildError)
        } else {
            Ok(InventoryChangeBody {
                idempotency_key: Uuid::new_v4().to_string(),
                changes: self.changes,
                ignore_unchanged_counts: self.ignore_unchanged_counts,
            })
        }
    }
}

pub struct InventoryChangeObjectBuilder {
    inventory_change_body_builder: InventoryChangeBodyBuilder,
    inventory_change: InventoryChange,
}

impl InventoryChangeObjectBuilder {
    pub fn change_type(mut self, change_type: InventoryChangeType) -> Self {
        self.inventory_change.inventory_change_type = change_type;
        
        self
    }
    
    pub fn physical_count(mut self, physical_count: InventoryPhysicalCount) -> Self {
        self.inventory_change.physical_count = Some(physical_count);
        
        self
    }

    pub fn adjustment(mut self, adjustment: InventoryAdjustment) -> Self {
        self.inventory_change.adjustment = Some(adjustment);

        self
    }

    pub fn transfer(mut self, transfer: InventoryTransfer) -> Self {
        self.inventory_change.transfer = Some(transfer);

        self
    }
    
    pub async fn into_body(mut self) -> InventoryChangeBodyBuilder {
        self.inventory_change_body_builder.change(self.inventory_change)
    }
}

#[cfg(test)]
mod test_inventory {
    use crate::objects::enums::InventoryState;
    use super::*;

    #[actix_rt::test]
    async fn test_retrieve_count() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);

        let input = (
            "DUHTRFG3OEYAXG5I22XLFN23".to_string(),
            "L1JC53TYHS40Z".to_string()
        );

        let res = sut.inventory()
            .retrieve_count(input.0, Some(input.1))
            .await;

        assert!(res.is_ok())
    }

    #[actix_rt::test]
    async fn test_change_body_builder() {
        let expected = InventoryChangeBody {
            idempotency_key: "".to_string(),
            changes: vec![
                InventoryChange {
                    adjustment: None,
                    measurement_unit: None,
                    measurement_unit_id: None,
                    physical_count: Some(InventoryPhysicalCount {
                        id: None,
                        catalog_object_id: "".to_string(),
                        catalog_object_type: None,
                        created_at: None,
                        location_id: "L1JC53TYHS40Z".to_string(),
                        occurred_at: "2022-07-09T12:25:34Z".to_string(),
                        quantity: "30".to_string(),
                        reference_id: None,
                        source: None,
                        state: InventoryState::InStock,
                        team_member_id: None
                    }),
                    transfer: None,
                    inventory_change_type: InventoryChangeType::PhysicalCount
                }
            ],
            ignore_unchanged_counts: None
        };

        let mut actual = InventoryChangeBodyBuilder::new()
            .change_object_builder()
            .change_type(InventoryChangeType::PhysicalCount)
            .physical_count(InventoryPhysicalCount {
                id: None,
                catalog_object_id: "".to_string(),
                catalog_object_type: None,
                created_at: None,
                location_id: "L1JC53TYHS40Z".to_string(),
                occurred_at: "2022-07-09T12:25:34Z".to_string(),
                quantity: "30".to_string(),
                reference_id: None,
                source: None,
                state: InventoryState::InStock,
                team_member_id: None
            })
            .into_body()
            .await
            .build()
            .await
            .unwrap();

        actual.idempotency_key = "".to_string();

        assert_eq!(format!("{:?}",expected), format!("{:?}",actual));
    }

    #[actix_rt::test]
    async fn test_batch_change() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);

        let input = InventoryChangeBody {
            idempotency_key: Uuid::new_v4().to_string(),
            changes: vec![
                InventoryChange {
                    adjustment: None,
                    measurement_unit: None,
                    measurement_unit_id: None,
                    physical_count: Some(InventoryPhysicalCount {
                        id: None,
                        catalog_object_id: "DUHTRFG3OEYAXG5I22XLFN23".to_string(),
                        catalog_object_type: None,
                        created_at: None,
                        location_id: "L1JC53TYHS40Z".to_string(),
                        occurred_at: "2022-07-09T12:25:34Z".to_string(),
                        quantity: "30".to_string(),
                        reference_id: None,
                        source: None,
                        state: InventoryState::InStock,
                        team_member_id: None
                    }),
                    transfer: None,
                    inventory_change_type: InventoryChangeType::PhysicalCount
                }
            ],
            ignore_unchanged_counts: None
        };

        let res = sut.inventory()
            .batch_change(input)
            .await;

        assert!(res.is_ok())
    }
}