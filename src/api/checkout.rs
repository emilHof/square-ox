/*!
Checkout functionality of the [Square API](https://developer.squareup.com).
 */

use crate::client::SquareClient;
use crate::api::{Verb, SquareAPI};
use crate::errors::{SquareError, ValidationError};
use crate::response::SquareResponse;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::builder::{AddField, Builder, ParentBuilder, Validate};
use crate::objects::{self, Address, ChargeRequestAdditionalRecipient, CheckoutOptions,
                     CreateOrderRequest, Order, PaymentLink, PrePopulatedData,
                     QuickPay};

impl SquareClient {
    pub fn checkout(&self) -> Checkout {
        Checkout {
            client: &self
        }
    }
}

pub struct Checkout<'a> {
    client: &'a SquareClient,
}

impl<'a> Checkout<'a> {
    /// Link a checkout id to a checkout page in order to redirect customers
    ///
    /// # Arguments:
    /// * `location_id` - The id of the location you would like to link to the checkout page.
    /// * `create_order_request`- The request body of the create_checkout call wrapped in a
    /// [CreateOrderRequestWrapper](CreateOrderRequestWrapper) created through the
    /// [CreateOrderRequestBuilder](CreateOrderRequestBuilder).
    pub async fn create_checkout(
        self, location_id: String,
        create_order_request: CreateOrderRequestWrapper
    )
        -> Result<SquareResponse, SquareError> {
        self.client.request(
            Verb::POST,
            SquareAPI::Locations(format!("/{}/checkouts", location_id)),
            Some(&create_order_request),
            None,
        ).await
    }

    /// Lists all payment links registered at the [Square API](https://developer.squareup.com).
    ///
    /// # Arguments:
    /// * `search_query` - The parameters restricting the listing of payment links. They are build
    /// through the [ListPaymentLinksSearchQueryBuilder](ListPaymentLinksSearchQueryBuilder) and in
    /// vector form.
    pub async fn list(
        self, search_query: Option<Vec<(String, String)>>
    )
        -> Result<SquareResponse, SquareError> {
        self.client.request(
            Verb::GET,
            SquareAPI::Checkout("/payment-links".to_string()),
            None::<&CreateOrderRequestWrapper>,
            search_query,
        ).await
    }

    /// Creates a Square-hosted checkout page. Applications can share the resulting payment link
    /// with their buyer to pay for goods and services.
    ///
    /// # Arguments:
    /// * `payment_link` - The body of the quest, holding the details of the payment link that is
    /// being added. This body is wrapped by a [CreatePaymentLinkWrapper](CreatePaymentLinkWrapper)
    /// that is created through the [CreatePaymentLinkBuilder](CreatePaymentLinkBuilder). The
    /// payment link must contain at least one Order or QuickPay object.
    pub async fn create(
        self, payment_link: CreatePaymentLinkWrapper
    )
        -> Result<SquareResponse, SquareError> {
        self.client.request(
            Verb::POST,
            SquareAPI::Checkout("/payment-links".to_string()),
            Some(&payment_link),
            None,
        ).await
    }

    /// Deletes a payment link.
    ///
    /// # Arguments:
    /// * `link_id` - The id of the payment link to delete.
    pub async fn delete(
        self, payment_link: String
    )
        -> Result<SquareResponse, SquareError> {
        self.client.request(
            Verb::DELETE,
            SquareAPI::Checkout(format!("/payment-links/{}", payment_link)),
            None::<&CreateOrderRequestWrapper>,
            None,
        ).await
    }

    /// Retrieves a payment link from the [Square API](https://developer.squareup.com).
    ///
    /// # Arguments:
    /// * `link_id` - The id of the payment link to delete.
    pub async fn retrieve(
        self, link_id: String
    )
        -> Result<SquareResponse, SquareError> {
        self.client.request(
            Verb::GET,
            SquareAPI::Checkout(format!("/payment-links/{}", link_id)),
            None::<&CreateOrderRequestWrapper>,
            None,
        ).await
    }

    /// Updates a payment link at the [Square API](https://developer.squareup.com).
    ///
    /// # Arguments:
    /// * `link_id` - The id of the payment link to update.
    /// * `payment_link` - The updated [PaymentLink](PaymentLink).
    pub async fn update(
        self, link_id: String, payment_link: UpdatePaymentLinkWrapper
    )
        -> Result<SquareResponse, SquareError> {
        self.client.request(
            Verb::PUT,
            SquareAPI::Checkout(format!("/payment-links/{}", link_id)),
            Some(&payment_link),
            None,
        ).await
    }
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CreateOrderRequestWrapper {
    idempotency_key: Option<String>,
    order: CreateOrderRequest,
    ask_for_shipping_address: Option<bool>,
    merchant_support_email: Option<String>,
    pre_populate_buyer_email: Option<bool>,
    pre_populate_shipping_address: Option<Address>,
    redirect_url: Option<String>,
    additional_recipients: Option<Vec<ChargeRequestAdditionalRecipient>>,
    note: Option<String>,
}

impl Default for CreateOrderRequestWrapper {
    fn default() -> Self {
        CreateOrderRequestWrapper {
            idempotency_key: None,
            order: CreateOrderRequest { idempotency_key: Uuid::new_v4().to_string(), order: Default::default() },
            ask_for_shipping_address: None,
            merchant_support_email: None,
            pre_populate_buyer_email: None,
            pre_populate_shipping_address: None,
            redirect_url: None,
            additional_recipients: None,
            note: None
        }
    }
}

impl Validate for CreateOrderRequestWrapper {
    fn validate(mut self) -> Result<Self, ValidationError> where Self: Sized {
        if self.order.order.location_id.is_some() {
            self.idempotency_key = Some(Uuid::new_v4().to_string());

            Ok(self)
        } else {
            Err(ValidationError)
        }
    }
}

impl<T: ParentBuilder> Builder<CreateOrderRequestWrapper, T> {
    pub fn order(mut self, order: Order) -> Self {
        self.body.order.order = order;

        self
    }

    pub fn ask_for_shipping_address(mut self) -> Self {
        self.body.ask_for_shipping_address = Some(true);

        self
    }

    pub fn merchant_support_email(mut self, merchant_support_email: String) -> Self {
        self.body.merchant_support_email = Some(merchant_support_email);

        self
    }

    pub fn redirect_url(mut self, redirect_url: String) -> Self {
        self.body.redirect_url = Some(redirect_url);

        self
    }

    pub fn pre_populate_buyer_email(mut self) -> Self {
        self.body.pre_populate_buyer_email = Some(true);

        self
    }
}

impl AddField<Order> for CreateOrderRequestWrapper {
    fn add_field(&mut self, field: Order) {
        self.order.order = field;
    }
}

#[derive(Default)]
pub struct ListPaymentLinksSearchQueryBuilder {
    cursor: Option<String>,
    limit: Option<i32>,
}

impl ListPaymentLinksSearchQueryBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn cursor(mut self, cursor: String) -> Self {
        self.cursor = Some(cursor);

        self
    }

    pub fn limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);

        self
    }

    pub async fn build(self) -> Vec<(String, String)> {
        let ListPaymentLinksSearchQueryBuilder {
            cursor,
            limit,
        } = self;

        let mut res = vec![];

        if let Some(cursor) = cursor {
            res.push(("cursor".to_string() , cursor));
        }

        if let Some(limit) = limit {
            res.push(("limit".to_string() , limit.to_string()));
        }

        res
    }
}

#[derive(Clone, Serialize, Debug, Default)]
pub struct CreatePaymentLinkWrapper {
    idempotency_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quick_pay: Option<QuickPay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order: Option<Order>,
    #[serde(skip_serializing_if = "Option::is_none")]
    checkout_options: Option<CheckoutOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pre_populated_data: Option<PrePopulatedData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_note: Option<String>,
}

impl Validate for CreatePaymentLinkWrapper {
    fn validate(mut self) -> Result<Self, ValidationError> where Self: Sized {
        if self.order.is_some() || self.quick_pay.is_some() {
            self.idempotency_key = Uuid::new_v4().to_string();

            Ok(self)
        } else {
            Err(ValidationError)
        }
    }
}

impl<T: ParentBuilder> Builder<CreatePaymentLinkWrapper, T> {
    pub fn checkout_options(mut self, checkout_options: CheckoutOptions) -> Self {
        self.body.checkout_options = Some(checkout_options);

        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.body.description = Some(description);

        self
    }

    pub fn order(mut self, order: Order) -> Self {
        self.body.order = Some(order);

        self
    }

    pub fn payment_note(mut self, payment_note: String) -> Self {
        self.body.payment_note = Some(payment_note);

        self
    }

    pub fn pre_populated_data(mut self, pre_populated_data: PrePopulatedData) -> Self {
        self.body.pre_populated_data = Some(pre_populated_data);

        self
    }

    pub fn quick_pay(mut self, quick_pay: QuickPay) -> Self {
        self.body.quick_pay = Some(quick_pay);

        self
    }

    pub fn source(mut self, source: String) -> Self {
        self.body.source = Some(source);

        self
    }
}

impl AddField<Order> for CreatePaymentLinkWrapper {
    fn add_field(&mut self, field: Order) {
        self.order = Some(field);
    }
}

#[derive(Clone, Serialize, Debug)]
pub struct UpdatePaymentLinkWrapper {
    payment_link: PaymentLink,
}

impl Default for UpdatePaymentLinkWrapper {
    fn default() -> Self {
        UpdatePaymentLinkWrapper {
            payment_link: PaymentLink {
                id: None,
                version: 1,
                checkout_options: None,
                created_at: None,
                description: None,
                order_id: None,
                payment_note: None,
                pre_populated_data: None,
                updated_at: None,
                url: None
            }
        }
    }
}

impl Validate for UpdatePaymentLinkWrapper {
    fn validate(self) -> Result<Self, ValidationError> where Self: Sized {
        if self.payment_link.version > 1 {
            Ok(self)
        } else {
            Err(ValidationError)
        }

    }
}

impl<T: ParentBuilder> Builder<UpdatePaymentLinkWrapper, T> {
    pub fn set_updated_payment_link(mut self, payment_link: PaymentLink) -> Self {
        self.body.payment_link = payment_link;

        self
    }

    pub fn version(mut self, version: i32) -> Self {
        self.body.payment_link.version = version;

        self
    }
}

#[cfg(test)]
mod test_checkout {
    use crate::builder::BackIntoBuilder;
    use crate::objects::{enums::{OrderLineItemItemType, Currency}, Money, OrderLineItem};
    use super::*;

    #[tokio::test]
    async fn test_create_order_request_builder() {
        let expected = CreateOrderRequestWrapper {
            idempotency_key: None,
            order: CreateOrderRequest { idempotency_key: "".to_string(), order: Order {
                id: None,
                location_id: Some("L1JC53TYHS40Z".to_string()),
                close_at: None,
                created_at: None,
                customer_id: None,
                discounts: None,
                fulfillments: None,
                line_items: Some(vec![
                    OrderLineItem {
                        quantity: "1".to_string(),
                        applied_discounts: None,
                        applied_taxes: None,
                        base_price_money: Some(Money {
                            amount: Some(5),
                            currency: Currency::USD
                        }),
                        catalog_object_id: Some("BSOL4BB6RCMX6SH4KQIFWZDP".to_string()),
                        catalog_version: Some(1655427266071),
                        gross_sales_money: None,
                        item_type: Some(OrderLineItemItemType::Item),
                        metadata: None,
                        modifiers: None,
                        name: None,
                        note: None,
                        pricing_blocklists: None,
                        quantity_unit: None,
                        total_discount_money: None,
                        total_money: None,
                        total_tax_money: None,
                        uid: None,
                        variation_name: None,
                        variation_total_price_money: None,
                        api_reference_ids: None
                    },
                    OrderLineItem {
                        quantity: "2".to_string(),
                        applied_discounts: None,
                        applied_taxes: None,
                        base_price_money: Some(Money {
                          amount: Some(5),
                          currency: Currency::USD
                        }),
                        catalog_object_id: Some("BSOL4BB6RCMX6SH4KQIFWZDP".to_string()),
                        catalog_version: Some(1655427266071),
                        gross_sales_money: None,
                        item_type: Some(OrderLineItemItemType::Item),
                        metadata: None,
                        modifiers: None,
                        name: None,
                        note: None,
                        pricing_blocklists: None,
                        quantity_unit: None,
                        total_discount_money: None,
                        total_money: None,
                        total_tax_money: None,
                        uid: None,
                        variation_name: None,
                        variation_total_price_money: None,
                        api_reference_ids: None
                    }]),
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
                version: None
            }},
            ask_for_shipping_address: None,
            merchant_support_email: None,
            pre_populate_buyer_email: None,
            pre_populate_shipping_address: None,
            redirect_url: None,
            additional_recipients: None,
            note: None
        };

        let mut actual = Builder::from(CreateOrderRequestWrapper::default())
            .sub_builder_from(Order::default())
            .add_order_item(OrderLineItem {
                quantity: "1".to_string(),
                applied_discounts: None,
                applied_taxes: None,
                base_price_money: Some(Money {
                    amount: Some(5),
                    currency: Currency::USD
                }),
                catalog_object_id: Some("BSOL4BB6RCMX6SH4KQIFWZDP".to_string()),
                catalog_version: Some(1655427266071),
                gross_sales_money: None,
                item_type: Some(OrderLineItemItemType::Item),
                metadata: None,
                modifiers: None,
                name: None,
                note: None,
                pricing_blocklists: None,
                quantity_unit: None,
                total_discount_money: None,
                total_money: None,
                total_tax_money: None,
                uid: None,
                variation_name: None,
                variation_total_price_money: None,
                api_reference_ids: None
            })
            .add_order_item(OrderLineItem {
                quantity: "2".to_string(),
                applied_discounts: None,
                applied_taxes: None,
                base_price_money: Some(Money {
                    amount: Some(5),
                    currency: Currency::USD
                }),
                catalog_object_id: Some("BSOL4BB6RCMX6SH4KQIFWZDP".to_string()),
                catalog_version: Some(1655427266071),
                gross_sales_money: None,
                item_type: Some(OrderLineItemItemType::Item),
                metadata: None,
                modifiers: None,
                name: None,
                note: None,
                pricing_blocklists: None,
                quantity_unit: None,
                total_discount_money: None,
                total_money: None,
                total_tax_money: None,
                uid: None,
                variation_name: None,
                variation_total_price_money: None,
                api_reference_ids: None
            })
            .location_id("L1JC53TYHS40Z".to_string())
            .into_parent_builder()
            .unwrap()
            .build()
            .await
            .unwrap();
        actual.idempotency_key = None;
        actual.order.idempotency_key = "".to_string();

        assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
    }

    #[tokio::test]
    async fn test_create_checkout() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);

        let input = CreateOrderRequestWrapper {
            idempotency_key: Some(Uuid::new_v4().to_string()),
            order: CreateOrderRequest { idempotency_key: Uuid::new_v4().to_string(), order: Order {
                id: None,
                location_id: Some("L1JC53TYHS40Z".to_string()),
                close_at: None,
                created_at: None,
                customer_id: None,
                discounts: None,
                fulfillments: None,
                line_items: Some(vec![
                    OrderLineItem {
                        quantity: "1".to_string(),
                        applied_discounts: None,
                        applied_taxes: None,
                        base_price_money: Some(Money {
                            amount: Some(5),
                            currency: Currency::USD
                        }),
                        catalog_object_id: Some("BSOL4BB6RCMX6SH4KQIFWZDP".to_string()),
                        catalog_version: Some(1655427266071),
                        gross_sales_money: None,
                        item_type: Some(OrderLineItemItemType::Item),
                        metadata: None,
                        modifiers: None,
                        name: None,
                        note: None,
                        pricing_blocklists: None,
                        quantity_unit: None,
                        total_discount_money: None,
                        total_money: None,
                        total_tax_money: None,
                        uid: None,
                        variation_name: None,
                        variation_total_price_money: None,
                        api_reference_ids: None
                    },
                    OrderLineItem {
                        quantity: "2".to_string(),
                        applied_discounts: None,
                        applied_taxes: None,
                        base_price_money: Some(Money {
                            amount: Some(5),
                            currency: Currency::USD
                        }),
                        catalog_object_id: Some("BSOL4BB6RCMX6SH4KQIFWZDP".to_string()),
                        catalog_version: Some(1655427266071),
                        gross_sales_money: None,
                        item_type: Some(OrderLineItemItemType::Item),
                        metadata: None,
                        modifiers: None,
                        name: None,
                        note: None,
                        pricing_blocklists: None,
                        quantity_unit: None,
                        total_discount_money: None,
                        total_money: None,
                        total_tax_money: None,
                        uid: None,
                        variation_name: None,
                        variation_total_price_money: None,
                        api_reference_ids: None
                    }]),
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
                version: None
            }},
            ask_for_shipping_address: None,
            merchant_support_email: None,
            pre_populate_buyer_email: None,
            pre_populate_shipping_address: None,
            redirect_url: None,
            additional_recipients: None,
            note: None
        };

        let res = sut.checkout()
            .create_checkout("L1JC53TYHS40Z".to_string(), input)
            .await;

        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_list_payment_search_query_builder() {
        let expected = vec![
            ("cursor".to_string(), "dwasd".to_string()),
            ("limit".to_string(), "10".to_string()),
        ];

        let actual = ListPaymentLinksSearchQueryBuilder::new()
            .limit(10)
            .cursor("dwasd".to_string())
            .build()
            .await;

        assert_eq!(expected, actual)
    }

    #[tokio::test]
    async fn test_list_payment_links() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);

        let input = vec![("limit".to_string(), "10".to_string())];

        let res = sut.checkout()
            .list(Some(input))
            .await;

        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_create_payment_link_builder() {
        let expected = CreatePaymentLinkWrapper {
            idempotency_key: "".to_string(),
            description: None,
            quick_pay: Some( QuickPay {
                location_id: "L1JC53TYHS40Z".to_string(),
                name: "Another Thing".to_string(),
                price_money: Money { amount: Some(10), currency: Currency::USD }
            }),
            order: None,
            checkout_options: None,
            pre_populated_data: None,
            source: None,
            payment_note: None
        };

        let mut actual = Builder::from(CreatePaymentLinkWrapper::default())
            .quick_pay(QuickPay {
                location_id: "L1JC53TYHS40Z".to_string(),
                name: "Another Thing".to_string(),
                price_money: Money { amount: Some(10), currency: Currency::USD }
            })
            .build()
            .await
            .unwrap();

        actual.idempotency_key = "".to_string();


        assert_eq!(format!("{:?}", expected), format!("{:?}", actual))
    }

    #[tokio::test]
    async fn test_create_payment_link() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);

        let input = CreatePaymentLinkWrapper {
            idempotency_key: "".to_string(),
            description: None,
            quick_pay: Some( QuickPay {
                location_id: "L1JC53TYHS40Z".to_string(),
                name: "Another Thing".to_string(),
                price_money: Money { amount: Some(10), currency: Currency::USD }
            }),
            order: None,
            checkout_options: None,
            pre_populated_data: None,
            source: None,
            payment_note: None
        };

        let res = sut.checkout()
            .create(input)
            .await;

        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_delete_payment_link() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);

        let input = "PLEJUTGT4VLUKUY2".to_string();

        let res = sut.checkout()
            .delete(input)
            .await;

        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_retrieve_payment_link() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);

        let input = "PN43H2RUILBXIX2H".to_string();

        let res = sut.checkout()
            .retrieve(input)
            .await;

        assert!(res.is_ok());
    }

    // #[tokio::test]
    async fn test_update_payment_link() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);

        let input = (
            "R6BRAXXKPCMYI2ZQ".to_string(),
            UpdatePaymentLinkWrapper {
                payment_link: objects::PaymentLink {
                    id: None,
                    version: 5,
                    checkout_options: None,
                    created_at: None,
                    description: None,
                    order_id: None,
                    payment_note: None,
                    pre_populated_data: None,
                    updated_at: None,
                    url: None
                }
            });

        let res = sut.checkout()
            .update(input.0, input.1)
            .await;

        assert!(res.is_ok());
    }
}
