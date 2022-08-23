/*!
Orders functionality of the [Square API](https://developer.squareup.com).
 */

use crate::api::{SquareAPI, Verb};
use crate::client::SquareClient;
use crate::errors::{SquareError, ValidationError};
use crate::objects::{Customer, Order, OrderReward, OrderServiceCharge, SearchOrdersQuery};
use crate::response::SquareResponse;
use crate::builder::{Builder, ParentBuilder, Validate, BackIntoBuilder, AddField, Buildable};
use square_ox_derive::Builder;

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

    /// Retrieves an [Order](Order) by ID.
    /// [Open in API Reference](https://developer.squareup.com/reference/square/orders/retrieve-order).
    pub async fn update(self, id: String, body: OrderUpdateBody)
                      -> Result<SquareResponse, SquareError> {
        self.client.request(
            Verb::PUT,
            SquareAPI::Orders(format!("/{}", id)),
            Some(&body),
            None,
        ).await
    }

    /// Pay for an [Order](Order) using one or more approved payments or settle an order with a
    /// total of 0.
    /// [Open in API Reference](https://developer.squareup.com/reference/square/orders/pay-order).
    pub async fn pay(self, id: String, body: PayOrderBody)
                      -> Result<SquareResponse, SquareError> {
        self.client.request(
            Verb::POST,
            SquareAPI::Orders(format!("/{}/pay", id)),
            Some(&body),
            None,
        ).await
    }

    /// Enables applications to preview [Order](Order) pricing without creating an order.
    /// [Open in API Reference](https://developer.squareup.com/reference/square/orders/calculate-order).
    pub async fn calculate(self, body: OrderCalculateBody)
                      -> Result<SquareResponse, SquareError> {
        self.client.request(
            Verb::POST,
            SquareAPI::Orders("/calculate".to_string()),
            Some(&body),
            None,
        ).await
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, Builder)]
pub struct CreateOrderBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder_rand("uuid")]
    #[builder_vis("private")]
    idempotency_key: Option<String>,
    order: Order,
}

impl AddField<Order> for CreateOrderBody {
    fn add_field(&mut self, field: Order) {
        self.order = field;
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Builder)]
pub struct SearchOrderBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder_into]
    location_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query: Option<SearchOrdersQuery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    return_entries: Option<bool>
}

impl Default for SearchOrderBody {
    fn default() -> Self {
        SearchOrderBody {
            cursor: None,
            limit: None,
            location_ids: None,
            query: None,
            return_entries: Some(true)
        }
    }
}

// implements the necessary traits to release an SearchOrdersQuery builder from a SearchOrderBody
// builder
impl AddField<SearchOrdersQuery> for SearchOrderBody {
    fn add_field(&mut self, field: SearchOrdersQuery) {
        self.query = Some(field);
    }
}

#[derive(Clone, Debug, Serialize, Default, Builder)]
pub struct OrderUpdateBody {
    fields_to_clear: Option<Vec<String>>,
    #[builder_rand("uuid")]
    idempotency_key: Option<String>,
    #[builder_validate("is_some")]
    order: Option<Order>,
}

// implements the necessary traits to release an Order builder from a OrderUpdateBody
// builder
impl AddField<Order> for OrderUpdateBody {
    fn add_field(&mut self, field: Order) {
        self.order = Some(field);
    }
}

#[derive(Clone, Debug, Serialize, Default, Builder)]
pub struct PayOrderBody {
    #[builder_rand("uuid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    idempotency_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder_validate("is_some")]
    order_version: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder_validate("is_some")]
    payment_ids: Option<Vec<String>>,
}

#[derive(Clone, Debug, Serialize, Default, Builder)]
pub struct OrderCalculateBody {
    #[builder_validate("is_some")]
    #[serde(skip_serializing_if = "Option::is_none")]
    order: Option<Order>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proposed_rewards: Option<Vec<OrderReward>>,
}

// implements the necessary traits to release an Order builder from a OrderCalculateBody
// builder
impl AddField<Order> for OrderCalculateBody {
    fn add_field(&mut self, field: Order) {
        self.order = Some(field)
    }
}

// implements the necessary traits to release an OrderReward builder from a OrderCalculateBody
// builder
impl AddField<OrderReward> for OrderCalculateBody {
    fn add_field(&mut self, field: OrderReward) {
        match self.proposed_rewards.as_mut() {
            Some(rewards) => rewards.push(field),
            None => self.proposed_rewards = Some(vec![field])
        }
    }
}

#[cfg(test)]
mod test_orders {
    use crate::builder::Nil;
    use crate::objects;
    use crate::objects::enums::{Currency, OrderServiceChargeCalculationPhase, SortOrder, SearchOrdersSortField};
    use crate::objects::{Money, SearchOrdersSort};
    use super::*;

    #[tokio::test]
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
                .sub_builder_from(Order::default())
                .location_id("location_id")
                .customer_id("customer_id".to_string())
                .sub_builder_from(OrderServiceCharge::default())
                .amount_money(Money { amount: Some(10), currency: Currency::USD })
                .name("some name")
                .calculation_phase(OrderServiceChargeCalculationPhase::TotalPhase)
                .build()
                .unwrap()
                .build()
                .unwrap()
                .build()
                .unwrap();

        assert!(actual.idempotency_key.is_some());

        actual.idempotency_key = None;

        assert_eq!(format!("{:?}", expected), format!("{:?}", actual))
    }

    #[tokio::test]
    async fn test_create_order_body_builder_fail() {
        let actual = Builder::from(CreateOrderBody::default())
            .sub_builder_from(Order::default())
            .location_id("location_id".to_string())
            .customer_id("customer_id".to_string())
            .sub_builder_from(OrderServiceCharge::default())
            .amount_money(Money { amount: Some(10), currency: Currency::USD })
            .calculation_phase(OrderServiceChargeCalculationPhase::TotalPhase)
            .build();

        assert!(actual.is_err());
    }

    #[tokio::test]
    async fn test_create_order() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);

        let input = CreateOrderBody {
            idempotency_key: None,
            order: objects::Order {
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

    #[tokio::test]
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
            return_entries: Some(true)
        };

        let actual = Builder::from(SearchOrderBody::default())
            .location_ids(
                vec![
                    "e23icos".to_string(),
                    "daiooaa".to_string(),
                    "pßasmxaskm".to_string(),
                ]
            )
            .limit(10)
            .sub_builder_from(SearchOrdersQuery::default())
            .sort_ascending()
            .build()
            .unwrap()
            .build()
            .unwrap();

        assert_eq!(format!("{:?}", expected), format!("{:?}", actual))
    }

    #[tokio::test]
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

    #[tokio::test]
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

    #[tokio::test]
    async fn test_update_order_body_fail() {

        let res_vec = vec![
            Builder::from(OrderUpdateBody::default())
                .fields_to_clear(vec!["a_field".to_string(), "another_field".to_string()])
                .sub_builder_from(Order::default())
                .version(2)
                .build(),
        ];

        res_vec.into_iter().for_each(|res| assert!(res.is_err()))
    }

    // #[tokio::test]
    async fn test_update_order() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);

        let input = OrderUpdateBody {
            fields_to_clear: None,
            idempotency_key: Some(Uuid::new_v4().to_string()),
            order: Some(Order {
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
                service_charges: None,
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
                version: Some(2)
            })
        };

        println!("{:?}", &input);

        let res = sut.orders()
            .update("TJn1daLZuaMmPGL8vbeFGSdxB9HZY".to_string(), input)
            .await;

        assert!(res.is_ok())
    }

    #[tokio::test]
    async fn test_pay_order_body_builder() {

        let expected = PayOrderBody {
            idempotency_key: None,
            order_version: Some(3),
            payment_ids: Some(vec!["some_id".to_string()])
        };

        let mut actual = Builder::from(PayOrderBody::default())
            .order_version(3)
            .payment_ids(vec!["some_id".to_string()])
            .build()
            .unwrap();

        assert!(actual.idempotency_key.is_some());

        actual.idempotency_key = None;

        assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
    }

    #[tokio::test]
    async fn test_order_calculate_body_builder() {

        let expected = OrderCalculateBody {
            order: Some(Order {
                id: None,
                location_id: Some("location_id".to_string()),
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
                service_charges: Some(vec![
                    OrderServiceCharge {
                        amount_money: Some(Money {
                            amount: Some(20),
                            currency: Currency::USD
                        }),
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
                    }
                ]),
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
                version: Some(3)
            }),
            proposed_rewards: None
        };

        let mut actual = Builder::from(OrderCalculateBody::default())
            .sub_builder_from(Order::default())
            .location_id("location_id".to_string())
            .sub_builder_from(OrderServiceCharge::default())
            .amount_money(Money { amount: Some(20), currency: Currency::USD })
            .name("some name".to_string())
            .calculation_phase(OrderServiceChargeCalculationPhase::TotalPhase)
            .build()
            .unwrap()
            .version(3)
            .build()
            .unwrap()
            .build()
            .unwrap();

        assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
    }

    #[tokio::test]
    async fn test_calculate_order() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);

        let input = OrderCalculateBody {
            order: Some(Order {
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
                service_charges: Some(vec![
                    OrderServiceCharge {
                        amount_money: Some(Money {
                            amount: Some(20),
                            currency: Currency::USD
                        }),
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
                    }
                ]),
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
                version: Some(3)
            }),
            proposed_rewards: None
        };

        let res = sut.orders()
            .calculate(input)
            .await;

        assert!(res.is_ok())
    }
}

