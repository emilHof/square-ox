/*!
Orders functionality of the [Square API](https://developer.squareup.com).
 */

use crate::api::{SquareAPI, Verb};
use crate::client::SquareClient;
use crate::errors::{SquareError, ValidationError};
use crate::objects::{Customer, Order, OrderReward, OrderServiceCharge, SearchOrdersQuery};
use crate::response::SquareResponse;
use crate::builder::{Builder, ParentBuilder, Nil, Validate, BackIntoBuilder, AddField};

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

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct CreateOrderBody {
    #[serde(skip_serializing_if = "Option::is_none")]
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
impl AddField<OrderServiceCharge> for CreateOrderBody {
    fn add_field(&mut self, field: OrderServiceCharge) {
        if let Some(service_charges) = self.order.service_charges.as_mut() {
            service_charges.push(field);
        } else {
            self.order.service_charges = Some(vec![field]);
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SearchOrderBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query: Option<SearchOrdersQuery>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
impl AddField<SearchOrdersQuery> for SearchOrderBody {
    fn add_field(&mut self, field: SearchOrdersQuery) {
        self.query = Some(field);
    }
}

#[derive(Clone, Debug, Serialize, Default)]
pub struct OrderUpdateBody {
    fields_to_clear: Option<Vec<String>>,
    idempotency_key: Option<String>,
    order: Option<Order>,
}

impl Validate for OrderUpdateBody {
    fn validate(mut self) -> Result<Self, ValidationError> where Self: Sized {
        if self.order.is_none() {
            Err(ValidationError)
        } else {
            self.idempotency_key = Some(Uuid::new_v4().to_string());

            Ok(self)
        }
    }
}

impl<T: ParentBuilder> Builder<OrderUpdateBody, T> {
    pub fn fields_to_clear(mut self, fields: Vec<String>) -> Self {
        self.body.fields_to_clear = Some(fields);

        self
    }

    pub fn order(mut self, order: Order) -> Self {
        self.body.order = Some(order);

        self
    }
}

// implements the necessary traits to release an Order builder from a OrderUpdateBody
// builder
impl AddField<Order> for OrderUpdateBody {
    fn add_field(&mut self, field: Order) {
        self.order = Some(field);
    }
}

#[derive(Clone, Debug, Serialize, Default)]
pub struct PayOrderBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    idempotency_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order_version: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_ids: Option<Vec<String>>,
}

impl Validate for PayOrderBody {
    fn validate(mut self) -> Result<Self, ValidationError> where Self: Sized {
        self.idempotency_key = Some(Uuid::new_v4().to_string());

        if self.order_version.is_some() &&
            self.payment_ids.is_some() {
            Ok(self)
        } else {
            Err(ValidationError)
        }


    }
}

impl<T: ParentBuilder> Builder<PayOrderBody, T> {
    fn oder_version(mut self, version: i64) -> Self {
        self.body.order_version = Some(version);

        self
    }

    fn payment_ids(mut self, ids: Vec<String>) -> Self {
        self.body.payment_ids = Some(ids);

        self
    }
}

#[derive(Clone, Debug, Serialize, Default)]
pub struct OrderCalculateBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    order: Option<Order>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proposed_rewards: Option<Vec<OrderReward>>,
}

impl Validate for OrderCalculateBody {
    fn validate(mut self) -> Result<Self, ValidationError> where Self: Sized {
        if self.order.is_some() {
            Ok(self)
        } else {
            Err(ValidationError)
        }
    }
}

impl<T: ParentBuilder> Builder<OrderCalculateBody, T> {
    fn order(mut self, order: Order) -> Self {
        self.body.order = Some(order);

        self
    }

    fn add_proposed_reward(mut self, reward: OrderReward) -> Self {
        match self.body.proposed_rewards.as_mut() {
            Some(rewards) => rewards.push(reward),
            None => self.body.proposed_rewards = Some(vec![reward])
        }

        self
    }
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
    use crate::api::bookings::QueryBody;
    use crate::builder::Nil;
    use crate::objects;
    use crate::objects::enums::{Currency, OrderServiceChargeCalculationPhase, SortOrder, SearchOrdersSortField};
    use crate::objects::{Money, SearchOrdersSort};
    use super::*;

    #[test]
    fn test_builder_from_create_order_body() {
        let expected = Builder{ body: CreateOrderBody::default(), parent_builder: None::<Nil> };

        let actual = Builder::from(CreateOrderBody::default());
    }

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
            .location_id("location_id".to_string())
            .customer_id("customer_id".to_string())
            .sub_builder_from(OrderServiceCharge::default())
            .amount_money(Money { amount: Some(10), currency: Currency::USD })
            .name("some name".to_string())
            .total_phase()
            .into_parent_builder()
            .unwrap()
            .build()
            .await
            .unwrap();

        assert!(actual.idempotency_key.is_some());

        actual.idempotency_key = None;

        assert_eq!(format!("{:?}", expected), format!("{:?}", actual))
    }

    #[tokio::test]
    async fn test_create_order_body_builder_fail() {
        let mut actual = Builder::from(CreateOrderBody::default())
            .location_id("location_id".to_string())
            .customer_id("customer_id".to_string())
            .sub_builder_from(OrderServiceCharge::default())
            .amount_money(Money { amount: Some(10), currency: Currency::USD })
            .total_phase()
            .into_parent_builder();

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

        let mut actual = Builder::from(SearchOrderBody::default())
            .add_location_id("e23icos".to_string())
            .add_location_id("daiooaa".to_string())
            .add_location_id("pßasmxaskm".to_string())
            .limit(10)
            .sub_builder_from(SearchOrdersQuery::default())
            .sort_ascending()
            .into_parent_builder()
            .unwrap()
            .no_return_entries()
            .build()
            .await
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
                .into_parent_builder(),
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
            .oder_version(3)
            .payment_ids(vec!["some_id".to_string()])
            .build()
            .await
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
            .total_phase()
            .into_parent_builder()
            .unwrap()
            .version(3)
            .into_parent_builder()
            .unwrap()
            .build()
            .await
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

