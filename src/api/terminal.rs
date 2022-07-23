/*!
Terminals functionality of the [Square API](https://developer.squareup.com).
 */

use crate::api::{SquareAPI, Verb};
use crate::client::SquareClient;
use crate::errors::{SquareError, ValidationError};
use crate::objects::{DeviceCheckoutOptions, Money, PaymentOptions, TerminalCheckout,
                     TerminalCheckoutQuery, TerminalRefund, TerminalRefundQuery};
use crate::objects::enums::{CheckoutOptionsPaymentType, TerminalCheckoutStatus};
use crate::response::SquareResponse;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::objects::TimeRange;
use crate::builder::{AddField, Builder, ParentBuilder, Validate};

impl SquareClient {
    pub fn terminal(&self) -> Terminal {
        Terminal {
            client: &self
        }
    }
}

pub struct Terminal<'a> {
    client: &'a SquareClient,
}

impl<'a> Terminal<'a> {
    /// Creates a Terminal checkout request and sends it to the specified device to take a payment
    /// for the requested amount.
    /// [Open in API Reference](https://developer.squareup.com/reference/square/terminal/create-terminal-checkout)
        pub async fn create_checkout(self, body: CreateTerminalCheckoutBody)
                              -> Result<SquareResponse, SquareError>{
        self.client.request(
            Verb::POST,
            SquareAPI::Terminals("/checkouts".to_string()),
            Some(&body),
            None,
        ).await
    }

    /// Returns a filtered list of Terminal checkout requests created by the application making the
    /// request. <br/>
    /// Only Terminal checkout requests created for the merchant scoped to the OAuth token are
    /// returned. Terminal checkout requests are available for 30 days.
    pub async fn search_checkout(self, body: SearchTerminalCheckoutBody)
                              -> Result<SquareResponse, SquareError>{
        self.client.request(
            Verb::GET,
            SquareAPI::Terminals("/checkouts/search".to_string()),
            Some(&body),
            None,
        ).await
    }

    /// Retrieves a Terminal checkout request by `checkout_id`.<br/>
    /// Terminal checkout requests are available for 30 days.
    pub async fn get_checkout(self, checkout_id: String)
                              -> Result<SquareResponse, SquareError>{
        self.client.request(
            Verb::GET,
            SquareAPI::Terminals(format!("/checkouts/{}", checkout_id)),
            None::<&CreateTerminalCheckoutBody>,
            None,
        ).await
    }

    /// Cancels a Terminal checkout request if the status of the request permits it.
    pub async fn cancel_checkout(self, checkout_id: String)
                              -> Result<SquareResponse, SquareError>{
        self.client.request(
            Verb::POST,
            SquareAPI::Terminals(format!("/checkouts/{}/cancel", checkout_id)),
            None::<&CreateTerminalCheckoutBody>,
            None,
        ).await
    }

    /// Creates a request to refund an Interac payment completed on a Square Terminal. <br/>
    /// Refunds for Interac payments on a Square Terminal are supported only for Interac debit cards
    /// in Canada. Other refunds for Terminal payments should use the Refunds API. For more
    /// information, see [Refunds API](https://developer.squareup.com/reference/square/refunds-api).
    pub async fn create_refund(self, body: CreateTerminalRefundBody)
                              -> Result<SquareResponse, SquareError>{
        self.client.request(
            Verb::POST,
            SquareAPI::Terminals("/refunds".to_string()),
            Some(&body),
            None,
        ).await
    }

    /// Retrieves a filtered list of Interac Terminal refund requests created by the seller making
    /// the request.
    /// [Open in API Reference](https://developer.squareup.com/reference/square/terminal/search-terminal-refunds)
    pub async fn search_refunds(self, body: SearchTerminalRefundBody)
                              -> Result<SquareResponse, SquareError>{
        self.client.request(
            Verb::POST,
            SquareAPI::Terminals("/refunds/search".to_string()),
            Some(&body),
            None,
        ).await
    }

    /// Retrieves an Interac Terminal refund object by ID.
    /// [Open in API Reference](https://developer.squareup.com/reference/square/terminal/get-terminal-refund)
    pub async fn get_refund(self, terminal_refund_id: String)
                              -> Result<SquareResponse, SquareError>{
        self.client.request(
            Verb::GET,
            SquareAPI::Terminals(format!("/refunds/{}", terminal_refund_id)),
            None::<&CreateTerminalRefundBody>,
            None,
        ).await
    }

    /// Cancels an Interac Terminal refund request by refund request ID if the status of the request
    /// permits it.
    /// [Open in API Reference](https://developer.squareup.com/reference/square/terminal/cancel-terminal-refund)
    pub async fn cancel_refund(self, terminal_refund_id: String)
                              -> Result<SquareResponse, SquareError>{
        self.client.request(
            Verb::POST,
            SquareAPI::Terminals(format!("/refunds/{}/cancel", terminal_refund_id)),
            None::<&CreateTerminalRefundBody>,
            None,
        ).await
    }
}

// -------------------------------------------------------------------------------------------------
// CreateTerminalCheckoutBody builder implementation
// -------------------------------------------------------------------------------------------------
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateTerminalCheckoutBody {
    idempotency_key: Option<String>,
    checkout: TerminalCheckout,
}

impl Default for CreateTerminalCheckoutBody {
    fn default() -> Self {
        CreateTerminalCheckoutBody {
            idempotency_key: None,
            checkout: TerminalCheckout::default(),
        }
    }
}

impl Validate for CreateTerminalCheckoutBody {
    fn validate(mut self) -> Result<Self, ValidationError> where Self: Sized {
        if self.checkout.amount_money.is_some() &&
            self.checkout.device_options.is_some() {
            self.idempotency_key = Some(Uuid::new_v4().to_string());
            Ok(self)
        } else {
            Err(ValidationError)
        }
    }
}

impl<T: ParentBuilder> Builder<CreateTerminalCheckoutBody, T> {
    pub fn amount_money(mut self, amount: Money) -> Self {
        self.body.checkout.amount_money = Some(amount);

        self
    }

    pub fn device_options(mut self, device_options: DeviceCheckoutOptions) -> Self {
        self.body.checkout.device_options = Some(device_options);

        self
    }

    pub fn customer_id(mut self, customer_id: String) -> Self {
        self.body.checkout.customer_id = Some(customer_id);

        self
    }

    pub fn deadline_duration(mut self, deadline_duration: String) -> Self {
        self.body.checkout.deadline_duration = Some(deadline_duration);

        self
    }

    pub fn note(mut self, note: String) -> Self {
        self.body.checkout.note = Some(note);

        self
    }

    pub fn order_id(mut self, order_id: String) -> Self {
        self.body.checkout.order_id = Some(order_id);

        self
    }

    pub fn payment_type(mut self, payment_type: CheckoutOptionsPaymentType) -> Self {
        self.body.checkout.payment_type = Some(payment_type);

        self
    }

    pub fn payment_options(mut self, payment_options: PaymentOptions) -> Self {
        self.body.checkout.payment_options = Some(payment_options);

        self
    }

    pub fn reference_id(mut self, reference_id: String) -> Self {
        self.body.checkout.reference_id = Some(reference_id);

        self
    }
}

impl AddField<DeviceCheckoutOptions> for CreateTerminalCheckoutBody {
    fn add_field(&mut self, field: DeviceCheckoutOptions) {
        self.checkout.device_options = Some(field);
    }
}

// -------------------------------------------------------------------------------------------------
// SearchTerminalCheckoutBody builder implementation
// -------------------------------------------------------------------------------------------------
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SearchTerminalCheckoutBody {
    query: Option<TerminalCheckoutQuery>,
    cursor: Option<String>,
    limit: Option<i32>,
}

impl Validate for SearchTerminalCheckoutBody {
    fn validate(self) -> Result<Self, ValidationError> where Self: Sized {
        Ok(self)
    }
}

impl<T: ParentBuilder> Builder<SearchTerminalCheckoutBody, T> {
    pub fn query(mut self, query: TerminalCheckoutQuery) -> Self {
        self.body.query = Some(query);

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
}

impl AddField<TerminalCheckoutQuery> for SearchTerminalCheckoutBody {
    fn add_field(&mut self, field: TerminalCheckoutQuery) {
        self.query = Some(field);
    }
}

// -------------------------------------------------------------------------------------------------
// CreateTerminalRefundBody builder implementation
// -------------------------------------------------------------------------------------------------
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct CreateTerminalRefundBody {
    idempotency_key: Option<String>,
    refund: TerminalRefund,
}

impl Validate for CreateTerminalRefundBody {
    fn validate(mut self) -> Result<Self, ValidationError> where Self: Sized {
        if self.refund.device_id.is_some() &&
            self.refund.amount_money.is_some() &&
            self.refund.reason.is_some() &&
            self.refund.payment_id.is_some() {
            self.idempotency_key = Some(Uuid::new_v4().to_string());

            Ok(self)
        } else {
            Err(ValidationError)
        }
    }
}

impl<T: ParentBuilder> Builder<CreateTerminalRefundBody, T> {
    pub fn amount_money(mut self, amount_money: Money) -> Self {
        self.body.refund.amount_money = Some(amount_money);

        self
    }

    pub fn device_id(mut self, device_id: String) -> Self {
        self.body.refund.device_id = Some(device_id);

        self
    }

    pub fn payment_id(mut self, payment_id: String) -> Self {
        self.body.refund.payment_id = Some(payment_id);

        self
    }

    pub fn reason(mut self, reason: String) -> Self {
        self.body.refund.reason = Some(reason);

        self
    }

    pub fn deadline_duration(mut self, reason: String) -> Self {
        self.body.refund.deadline_duration = Some(reason);

        self
    }
}

// -------------------------------------------------------------------------------------------------
// SearchTerminalRefundBody builder implementation
// -------------------------------------------------------------------------------------------------
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SearchTerminalRefundBody {
    cursor: Option<String>,
    limit: Option<i32>,
    query: Option<TerminalRefundQuery>,
}

impl Validate for SearchTerminalRefundBody {
    fn validate(self) -> Result<Self, ValidationError> where Self: Sized {
        Ok(self)
    }
}

impl<T: ParentBuilder> Builder<SearchTerminalRefundBody, T> {
    pub fn query(mut self, query: TerminalRefundQuery) -> Self {
        self.body.query = Some(query);

        self
    }

    pub fn limit(mut self, limit: i32) -> Self {
        self.body.limit = Some(limit);

        self
    }

    pub fn cursor(mut self, cursor: String) -> Self {
        self.body.cursor = Some(cursor);

        self
    }
}

impl AddField<TerminalRefundQuery> for SearchTerminalRefundBody {
    fn add_field(&mut self, field: TerminalRefundQuery) {
        self.query = Some(field);
    }
}

#[cfg(test)]
mod test_terminals {
    use crate::builder::BackIntoBuilder;
    use super::*;
    use crate::objects::enums::{Currency, SortOrder};
    use crate::objects::{TerminalCheckoutQueryFilter, TerminalCheckoutQuerySort};

    #[tokio::test]
    async fn test_create_terminal_checkout_body_builder() {
        let expected = CreateTerminalCheckoutBody {
            idempotency_key: None,
            checkout: TerminalCheckout {
                id: None,
                amount_money: Some(Money {
                    amount: Some(10),
                    currency: Currency::USD
                }),
                device_options: Some(DeviceCheckoutOptions {
                    device_id: Some("some_id".to_string()),
                    collect_signature: Some(true),
                    show_itemized_cart: None,
                    skip_receipt_screen: Some(true),
                    tip_settings: None
                }),
                app_fee_money: None,
                app_id: None,
                cancel_reason: None,
                created_at: None,
                customer_id: None,
                deadline_duration: None,
                location_id: None,
                note: None,
                order_id: None,
                payment_ids: None,
                payment_options: None,
                payment_type: None,
                reference_id: None,
                status: None,
                updated_at: None
            }
        };

        let mut actual = Builder::from(CreateTerminalCheckoutBody::default())
            .amount_money(Money { amount: Some(10), currency: Currency::USD })
            .sub_builder_from(DeviceCheckoutOptions::default())
            .device_id("some_id".to_string())
            .collect_signature()
            .skip_receipt_screen()
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
    async fn test_search_terminal_checkout_body_builder() {
        let expected = SearchTerminalCheckoutBody {
            query: Some(TerminalCheckoutQuery {
                filter: Some(TerminalCheckoutQueryFilter {
                    created_at: None,
                    device_id: Some("some_id".to_string()),
                    status: None
                }),
                sort: Some(TerminalCheckoutQuerySort {
                    sort_order: Some(SortOrder::Asc)
                })
            }),
            cursor: None,
            limit: Some(10)
        };

        let actual = Builder::from(SearchTerminalCheckoutBody::default())
            .limit(10)
            .sub_builder_from(TerminalCheckoutQuery::default())
            .sort_ascending()
            .device_id("some_id".to_string())
            .into_parent_builder()
            .unwrap()
            .build()
            .await
            .unwrap();

        assert_eq!(format!("{:?}", expected), format!("{:?}", actual))
    }

    // #[tokio::test]
    async fn test_search_checkouts() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);

        let input = SearchTerminalCheckoutBody {
            query: Some(TerminalCheckoutQuery {
                filter: Some(TerminalCheckoutQueryFilter {
                    created_at: None,
                    device_id: None,
                    status: Some(TerminalCheckoutStatus::Completed)
                }),
                sort: Some(TerminalCheckoutQuerySort {
                    sort_order: Some(SortOrder::Asc)
                })
            }),
            cursor: None,
            limit: Some(10)
        };

        let res = sut.terminal()
            .search_checkout(input)
            .await;

        assert!(res.is_err())
    }

    #[tokio::test]
    async fn test_create_terminal_refund_body_builder() {
        let expected = CreateTerminalRefundBody {
            idempotency_key: None,
            refund: TerminalRefund {
                id: None,
                amount_money: Some(Money {
                    amount: Some(10),
                    currency: Currency::USD
                }),
                device_id: Some("some_id".to_string()),
                payment_id: Some("some_id".to_string()),
                reason: Some("some reason".to_string()),
                app_id: None,
                cancel_reason: None,
                created_at: None,
                deadline_duration: None,
                location_id: None,
                order_id: None,
                refund_id: None,
                status: None,
                updated_at: None
            }
        };

        let mut actual = Builder::from(CreateTerminalRefundBody::default())
            .amount_money(Money { amount: Some(10), currency: Currency::USD })
            .device_id("some_id".to_string())
            .payment_id("some_id".to_string())
            .reason("some reason".to_string())
            .build()
            .await
            .unwrap();

        actual.idempotency_key = None;

        assert_eq!(format!("{:?}", expected), format!("{:?}", actual))
    }

    #[tokio::test]
    async fn test_create_terminal_refund_body_builder_fail() {

        let res = Builder::from(CreateTerminalRefundBody::default())
            .payment_id("some_id".to_string())
            .device_id("some_id".to_string())
            .amount_money(Money { amount: Some(10), currency: Currency::USD })
            .build()
            .await;

        assert!(res.is_err())
    }

    #[tokio::test]
    async fn test_search_terminal_refund_body_builder() {
        use crate::objects::TerminalRefundQueryFilter;
        let expected = SearchTerminalRefundBody {
            cursor: Some("some cursor".to_string()),
            limit: Some(10),
            query: Some(TerminalRefundQuery{
                filter: Some(TerminalRefundQueryFilter {
                    created_at: None,
                    device_id: Some("some_id".to_string()),
                    status: Some(TerminalCheckoutStatus::CancelRequested)
                }),
                sort: Some(TerminalCheckoutQuerySort { sort_order: Some(SortOrder::Desc) })
            })
        };

        let actual = Builder::from(SearchTerminalRefundBody::default())
            .limit(10)
            .cursor("some cursor".to_string())
            .sub_builder_from(TerminalRefundQuery::default())
            .device_id("some_id".to_string())
            .sort_descending()
            .cancel_requested()
            .into_parent_builder()
            .unwrap()
            .build()
            .await
            .unwrap();

        assert_eq!(format!("{:?}", expected), format!("{:?}", actual))
    }
}

