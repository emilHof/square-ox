/*!
Orders functionality of the [Square API](https://developer.squareup.com).
 */

use crate::api::{SquareAPI, Verb};
use crate::client::SquareClient;
use crate::errors::{SquareError, ValidationError};
use crate::objects::{Customer, Order, OrderServiceCharge, SearchOrdersQuery};
use crate::response::SquareResponse;
use crate::builder::{Builder, ParentBuilder, Nil, Validate, BackIntoBuilder};

use serde::{Serialize, Deserialize};
use uuid::Uuid;

impl SquareClient {
    pub fn orders(&self) -> Orders {
        Orders {
            client: &self,
        }
    }
}

pub struct Orders<'a> {
    client: &'a SquareClient
}

impl<'a> Orders<'a> {
    /// Creates a new [Order](Order) that can include information about products for purchase and settings
    /// to apply to the purchase.
    /// To pay for a created order, see [Pay for Orders](https://developer.squareup.com/docs/orders-api/pay-for-orders).
    pub async fn create(self, body: CreateOrderBody)
                      -> Result<SquareResponse, SquareError> {
        self.client.request(
            Verb::POST,
            SquareAPI::Orders("".to_string()),
            Some(&body),
            None,
        ).await
    }

    /// Search all orders for one or more locations.
    /// [Open in API Reference](https://developer.squareup.com/reference/square/orders/search-orders).
    pub async fn search(self, body: SearchOrderBody)
                      -> Result<SquareResponse, SquareError> {
        self.client.request(
            Verb::POST,
            SquareAPI::Orders("/search".to_string()),
            Some(&body),
            None,
        ).await
    }

    /// Retrieves an [Order](Order) by ID.
    /// [Open in API Reference](https://developer.squareup.com/reference/square/orders/retrieve-order).
    pub async fn retrieve(self, id: String)
                      -> Result<SquareResponse, SquareError> {
        self.client.request(
            Verb::GET,
            SquareAPI::Orders(format!("/{}", id)),
            None::<&SearchOrderBody>,
            None,
        ).await
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct CreateOrderBody {
    idempotency_key: Option<String>,
    order: Order,
}

impl Validate for CreateOrderBody {
    fn validate(mut self) -> Result<Self, ValidationError> {
        if self.order.location_id.is_some(){
            self.idempotency_key = Some(Uuid::new_v4().to_string());
            Ok(self)
        } else {
            Err(ValidationError)
        }
    }
}

impl<T: ParentBuilder> Builder<CreateOrderBody, T> {
    pub fn location_id(mut self, location_id: String) -> Self {
        self.body.order.location_id = Some(location_id);

        self
    }

    pub fn customer_id(mut self, customer_id: String) -> Self {
        self.body.order.customer_id = Some(customer_id);

        self
    }

    pub fn add_service_charge(mut self, service_charge: OrderServiceCharge) -> Self {
        if let Some(services_charges) = self.body.order.service_charges.as_mut() {
            services_charges.push(service_charge)
        } else {
            self.body.order.service_charges = Some(vec![service_charge])
        }

        self
    }
}

// implements the necessary traits to release an OrderServiceCharge builder from a CreateOrderBody
// builder
impl<T: ParentBuilder> BackIntoBuilder<OrderServiceCharge, Builder<CreateOrderBody, T>>
for  Builder<CreateOrderBody, T>
{
    fn add_field(self, field: OrderServiceCharge) -> Self {
        self.add_service_charge(field)
    }

    fn sub_builder_from(self, body: OrderServiceCharge)
        -> Builder<OrderServiceCharge, Builder<CreateOrderBody, T>> {
        Builder {
            body,
            builder: Some(self)
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SearchOrderBody {
    cursor: Option<String>,
    limit: Option<i32>,
    location_ids: Option<Vec<String>>,
    query: Option<SearchOrdersQuery>,
    return_entries: Option<bool>
}

impl Validate for SearchOrderBody {
    fn validate(mut self) -> Result<Self, ValidationError> {
        self.return_entries = Some(true);

        Ok(self)
    }
}

impl<T: ParentBuilder> Builder<SearchOrderBody, T> {
    pub fn add_location_id(mut self, id: String) -> Self {
        match self.body.location_ids.as_mut() {
            Some(ids) => ids.push(id),
            None => self.body.location_ids = Some(vec![id]),
        }

        self
    }

    pub fn location_ids(mut self, ids: Vec<String>) -> Self {
        self.body.location_ids = Some(ids);

        self
    }

    pub fn cursor(mut self, cursor: String) -> Self {
        self.body.cursor = Some(cursor);

        self
    }

    pub fn limit(mut self, limit: i32) -> Self {
        self.body.limit = Some(limit);

        self
    }

    pub fn no_return_entries(mut self) -> Self {
        self.body.return_entries = Some(false);

        self
    }

    pub fn query(mut self, query: SearchOrdersQuery) -> Self {
        self.body.query = Some(query);

        self
    }
}

// implements the necessary traits to release an SearchOrdersQuery builder from a SearchOrderBody
// builder
impl<T: ParentBuilder> BackIntoBuilder<SearchOrdersQuery, Builder<SearchOrderBody, T>>
for  Builder<SearchOrderBody, T>
{
    fn add_field(self, field: SearchOrdersQuery) -> Self {
        self.query(field)
    }

    fn sub_builder_from(self, body: SearchOrdersQuery)
                        -> Builder<SearchOrdersQuery, Builder<SearchOrderBody, T>> {
        Builder {
            body,
            builder: Some(self)
        }
    }
}

#[cfg(test)]
mod test_orders {
    use crate::api::bookings::QueryBody;
    use crate::builder::Nil;
    use crate::objects::enums::{Currency, OrderServiceChargeCalculationPhase, SortOrder, SearchOrdersSortField};
    use crate::objects::{Money, SearchOrdersSort};
    use super::*;

    #[test]
    fn test_builder_from_create_order_body() {
        let expected = Builder{ body: CreateOrderBody::default(), builder: None::<Nil> };

        let actual = Builder::from(CreateOrderBody::default());
    }

    #[actix_rt::test]
    async fn test_create_order_body_builder() {
        let expected = CreateOrderBody {
            idempotency_key: None,
            order: Order {
            id: None,
            location_id: Some("location_id".to_string()),
            close_at: None,
            created_at: None,
            customer_id: Some("customer_id".to_string()),
            discounts: None,
            fulfillments: None,
            line_items: None,
            metadata: None,
            net_amounts: None,
            pricing_options: None,
            reference_id: None,
            refunds: None,
            return_amounts: None,
            returns: None,
            rewards: None,
            rounding_adjustment: None,
            service_charges: Some(vec![OrderServiceCharge {
            amount_money: Some(Money{ amount: Some(10), currency: Currency::USD }),
            applied_money: None,
            applied_taxes: None,
            calculation_phase: Some(OrderServiceChargeCalculationPhase::TotalPhase),
            catalog_object_id: None,
            catalog_version: None,
            metadata: None,
            name: Some("some name".to_string()),
            percentage: None,
            taxable: None,
            total_money: None,
            total_tax_money: None,
            service_charge_type: None,
            uid: None
            }]),
            source: None,
            state: None,
            taxes: None,
            tenders: None,
            ticket_name: None,
            total_discount_money: None,
            total_money: None,
            total_service_charge_money: None,
            total_tax_money: None,
            total_tip_money: None,
            updated_at: None,
            version: None
            }
            };

            let mut actual = Builder::from(CreateOrderBody::default())
            .location_id("location_id".to_string())
            .customer_id("customer_id".to_string())
            .sub_builder_from(OrderServiceCharge::default())
            .amount_money(Money { amount: Some(10), currency: Currency::USD })
            .name("some name".to_string())
            .total_phase()
            .into_builder()
            .unwrap()
            .build()
            .await
            .unwrap();

        assert!(actual.idempotency_key.is_some());

        actual.idempotency_key = None;

        assert_eq!(format!("{:?}", expected), format!("{:?}", actual))
    }

    #[actix_rt::test]
    async fn test_create_order_body_builder_fail() {
        let mut actual = Builder::from(CreateOrderBody::default())
            .location_id("location_id".to_string())
            .customer_id("customer_id".to_string())
            .sub_builder_from(OrderServiceCharge::default())
            .amount_money(Money { amount: Some(10), currency: Currency::USD })
            .total_phase()
            .into_builder();

        assert!(actual.is_err());
    }

    #[actix_rt::test]
    async fn test_create_order() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);

        let input = CreateOrderBody {
            idempotency_key: None,
            order: Order {
                id: None,
                location_id: Some("L1JC53TYHS40Z".to_string()),
                close_at: None,
                created_at: None,
                customer_id: None,
                discounts: None,
                fulfillments: None,
                line_items: None,
                metadata: None,
                net_amounts: None,
                pricing_options: None,
                reference_id: None,
                refunds: None,
                return_amounts: None,
                returns: None,
                rewards: None,
                rounding_adjustment: None,
                service_charges: Some(vec![OrderServiceCharge {
                    amount_money: Some(Money{ amount: Some(15), currency: Currency::USD }),
                    applied_money: None,
                    applied_taxes: None,
                    calculation_phase: Some(OrderServiceChargeCalculationPhase::TotalPhase),
                    catalog_object_id: None,
                    catalog_version: None,
                    metadata: None,
                    name: Some("some name".to_string()),
                    percentage: None,
                    taxable: None,
                    total_money: None,
                    total_tax_money: None,
                    service_charge_type: None,
                    uid: None
                }]),
                source: None,
                state: None,
                taxes: None,
                tenders: None,
                ticket_name: None,
                total_discount_money: None,
                total_money: None,
                total_service_charge_money: None,
                total_tax_money: None,
                total_tip_money: None,
                updated_at: None,
                version: None
            }
        };

        let res = sut.orders()
            .create(input)
            .await;

        assert!(res.is_ok())
    }

    #[actix_rt::test]
    async fn test_search_order_body_builder() {
        let expected = SearchOrderBody {
            cursor: None,
            limit: Some(10),
            location_ids: Some(vec!["e23icos".to_string(), "daiooaa".to_string(), "pßasmxaskm".to_string()]),
            query: Some(SearchOrdersQuery {
                filter: None,
                sort: Some(SearchOrdersSort {
                    sort_field: Some(SearchOrdersSortField::CreatedAt),
                    sort_order: Some(SortOrder::Asc)
                })
            }),
            return_entries: Some(false)
        };

        let mut actual = Builder::from(SearchOrderBody::default())
            .add_location_id("e23icos".to_string())
            .add_location_id("daiooaa".to_string())
            .add_location_id("pßasmxaskm".to_string())
            .limit(10)
            .sub_builder_from(SearchOrdersQuery::default())
            .sort_ascending()
            .into_builder()
            .unwrap()
            .no_return_entries()
            .build()
            .await
            .unwrap();

        assert_eq!(format!("{:?}", expected), format!("{:?}", actual))
    }

    #[actix_rt::test]
    async fn test_search_orders() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);

        let input = SearchOrderBody {
            cursor: None,
            limit: None,
            location_ids: Some(vec!["L1JC53TYHS40Z".to_string()]),
            query: Some(SearchOrdersQuery {
                filter: None,
                sort: Some(SearchOrdersSort {
                    sort_field: Some(SearchOrdersSortField::CreatedAt),
                    sort_order: Some(SortOrder::Asc)
                })
            }),
            return_entries: Some(true)
        };

        let res = sut.orders()
            .search(input)
            .await;

        assert!(res.is_ok())
    }

    #[actix_rt::test]
    async fn test_retrieve_order() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);


        let res = sut.orders()
            .retrieve("HnbOXf4007VldqxbMvuzf0IjgyAZY".to_string())
            .await;

        assert!(res.is_ok())
    }
}

