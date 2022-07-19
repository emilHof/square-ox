/*!
Inventory functionality of the [Square API](https://developer.squareup.com).
 */

use crate::client::SquareClient;
use crate::api::{Verb, SquareAPI};
use crate::errors::{InventoryChangeBodyBuildError, SquareError, ValidationError};
use crate::response::SquareResponse;
use crate::objects::{CatalogObject, InventoryAdjustment, InventoryChange, InventoryPhysicalCount,
                     InventoryTransfer};
use crate::objects::enums::InventoryChangeType;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::builder::{AddField, Builder, ParentBuilder, Validate};


impl SquareClient {
    /// Returns an [Inventory](Inventory) object through which you can make calls specifically to
    /// the Inventory endpoint of the [Square API](https://developer.squareup.com).
    /// # Example: Using the inventory endpoint to make a retrieve_count request.
    /// ```rust
    /// use square_ox::{
    ///         response::{SquareResponse, ResponseError},
    ///         client::SquareClient,
    ///         api::inventory::Inventory,
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

    /// Retrieves the current calculated stock count for a given [CatalogObject](crate::objects::CatalogObject) at
    /// a given set of [Location](crate::objects::Location)s.
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

// -------------------------------------------------------------------------------------------------
// InventoryChangeBody builder implementation
// -------------------------------------------------------------------------------------------------
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct InventoryChangeBody {
    idempotency_key: Option<String>,
    changes: Vec<InventoryChange>,
    ignore_unchanged_counts: Option<bool>,
}

impl Validate for InventoryChangeBody {
    fn validate(mut self) -> Result<Self, ValidationError> where Self: Sized {
        if self.changes.len() > 0 {
            self.idempotency_key = Some(Uuid::new_v4().to_string());

            Ok(self)
        } else {
            Err(ValidationError)
        }
    }
}

impl<T: ParentBuilder> Builder<InventoryChangeBody, T> {
    pub fn change(mut self, change: InventoryChange) -> Self {
        self.body.changes.push(change);

        self
    }
}

impl AddField<InventoryChange> for InventoryChangeBody {
    fn add_field(&mut self, field: InventoryChange) {
        self.changes.push(field);
    }
}

#[cfg(test)]
mod test_inventory {
    use crate::builder::BackIntoBuilder;
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
            idempotency_key: None,
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

        let mut actual = Builder::from(InventoryChangeBody::default())
            .sub_builder_from(InventoryChange::default())
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
            .into_parent_builder()
            .unwrap()
            .build()
            .await
            .unwrap();

        assert!(actual.idempotency_key.is_some());

        actual.idempotency_key = None;

        assert_eq!(format!("{:?}",expected), format!("{:?}",actual));
    }

    // #[actix_rt::test]
    async fn test_batch_change() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);

        let input = InventoryChangeBody {
            idempotency_key: Some(Uuid::new_v4().to_string()),
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