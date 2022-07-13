/*!
Terminals functionality of the [Square API](https://developer.squareup.com).
 */

use crate::api::{SquareAPI, Verb};
use crate::client::SquareClient;
use crate::errors::{CreateTerminalCheckoutBodyBuildError, CreateTerminalRefundBodyBuildError, DeviceOptionsBuildError, SquareError};
use crate::objects::{DeviceCheckoutOptions, Money, PaymentOptions, TerminalCheckout, TerminalCheckoutQuery, TerminalCheckoutQueryFilter, TerminalCheckoutQuerySort, TerminalRefund, TerminalRefundQuery, TerminalRefundQueryFilter, TipSettings};
use crate::objects::enums::{CheckoutOptionsPaymentType, SortOrder, TerminalCheckoutStatus};
use crate::response::SquareResponse;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::api::customers::TimeRange;

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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateTerminalCheckoutBody {
    idempotency_key: String,
    checkout: TerminalCheckout,

}

pub struct CreateTerminalCheckoutBodyBuilder {
    checkout: TerminalCheckout,
}

impl CreateTerminalCheckoutBodyBuilder {
    pub fn new() -> Self {
        CreateTerminalCheckoutBodyBuilder {
            checkout: TerminalCheckout {
                id: None,
                amount_money: None,
                device_options: None,
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
        }
    }

    pub fn amount_money(mut self, amount: Money) -> Self {
        self.checkout.amount_money = Some(amount);

        self
    }

    pub fn device_options(mut self, device_options: DeviceCheckoutOptions) -> Self {
        self.checkout.device_options = Some(device_options);

        self
    }

    pub fn device_options_builder(self) -> DeviceOptionsBuilder {
        DeviceOptionsBuilder::new(self)
    }

    pub fn customer_id(mut self, customer_id: String) -> Self {
        self.checkout.customer_id = Some(customer_id);

        self
    }

    pub fn deadline_duration(mut self, deadline_duration: String) -> Self {
        self.checkout.deadline_duration = Some(deadline_duration);

        self
    }

    pub fn note(mut self, note: String) -> Self {
        self.checkout.note = Some(note);

        self
    }

    pub fn order_id(mut self, order_id: String) -> Self {
        self.checkout.order_id = Some(order_id);

        self
    }

    pub fn payment_type(mut self, payment_type: CheckoutOptionsPaymentType) -> Self {
        self.checkout.payment_type = Some(payment_type);

        self
    }

    pub fn payment_options(mut self, payment_options: PaymentOptions) -> Self {
        self.checkout.payment_options = Some(payment_options);

        self
    }

    pub fn reference_id(mut self, reference_id: String) -> Self {
        self.checkout.reference_id = Some(reference_id);

        self
    }

    pub async fn build(self)
        -> Result<CreateTerminalCheckoutBody, CreateTerminalCheckoutBodyBuildError> {
        if self.checkout.amount_money.is_none() ||
            self.checkout.device_options.is_none() {
            Err(CreateTerminalCheckoutBodyBuildError)
        } else {
            Ok(CreateTerminalCheckoutBody {
                idempotency_key: Uuid::new_v4().to_string(),
                checkout: self.checkout
            })
        }
    }
}

pub struct DeviceOptionsBuilder {
    builder: CreateTerminalCheckoutBodyBuilder,
    device_options: DeviceCheckoutOptions,
}

impl DeviceOptionsBuilder {
    pub fn new(builder: CreateTerminalCheckoutBodyBuilder) -> DeviceOptionsBuilder {
        DeviceOptionsBuilder {
            builder,
            device_options: DeviceCheckoutOptions {
                device_id: None,
                collect_signature: None,
                show_itemized_cart: None,
                skip_receipt_screen: None,
                tip_settings: None
            }
        }
    }

    pub fn device_id(mut self, device_id: String) -> Self {
        self.device_options.device_id = Some(device_id);

        self
    }

    pub fn collect_signature(mut self) -> Self {
        self.device_options.collect_signature = Some(true);

        self
    }

    pub fn show_itemized_cart(mut self) -> Self {
        self.device_options.show_itemized_cart = Some(true);

        self
    }

    pub fn skip_receipt_screen(mut self) -> Self {
        self.device_options.show_itemized_cart = Some(true);

        self
    }

    pub fn tip_settings(mut self, tip_settings: TipSettings) -> Self {
        self.device_options.tip_settings = Some(tip_settings);

        self
    }

    pub fn into_builder(self) -> Result<CreateTerminalCheckoutBodyBuilder, DeviceOptionsBuildError> {
        if self.device_options.device_id.is_none() {
            Err(DeviceOptionsBuildError)
        } else {
            Ok(self.builder.device_options(self.device_options))
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SearchTerminalCheckoutBody {
    query: Option<TerminalCheckoutQuery>,
    cursor: Option<String>,
    limit: Option<i32>,
}

#[derive(Default)]
pub struct SearchTerminalCheckoutBodyBuilder {
    body: SearchTerminalCheckoutBody
}

impl SearchTerminalCheckoutBodyBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn query(mut self, query: TerminalCheckoutQuery) -> Self {
        self.body.query = Some(query);

        self
    }

    pub fn query_builder(self) -> TerminalCheckoutQueryBuilder {
        TerminalCheckoutQueryBuilder::new(self)
    }

    pub fn cursor(mut self, cursor: String) -> Self {
        self.body.cursor = Some(cursor);

        self
    }

    pub fn limit(mut self, limit: i32) -> Self {
        self.body.limit = Some(limit);

        self
    }

    pub async fn build(self) -> SearchTerminalCheckoutBody {
        self.body
    }
}

pub struct TerminalCheckoutQueryBuilder {
    query: TerminalCheckoutQuery,
    builder: SearchTerminalCheckoutBodyBuilder,
}

impl TerminalCheckoutQueryBuilder {
    pub fn new(builder: SearchTerminalCheckoutBodyBuilder) -> Self {
        TerminalCheckoutQueryBuilder {
            query: TerminalCheckoutQuery { filter: None, sort: None },
            builder
        }
    }

    pub fn sort_ascending(mut self) -> Self {
        self.query.sort = Some(TerminalCheckoutQuerySort { sort_order: Some(SortOrder::Asc) });

        self
    }

    pub fn sort_descending(mut self) -> Self {
        self.query.sort = Some(TerminalCheckoutQuerySort { sort_order: Some(SortOrder::Desc) });

        self
    }

    pub fn created_at(mut self, created_at: TimeRange) -> Self {
        if let Some(filter) = self.query.filter.as_mut() {
            filter.created_at = Some(created_at);
        } else {
            self.query.filter = Some(TerminalCheckoutQueryFilter {
                created_at: Some(created_at),
                device_id: None,
                status: None
            })
        };

        self
    }

    pub fn device_id(mut self, device_id: String) -> Self {
        if let Some(filter) = self.query.filter.as_mut() {
            filter.device_id = Some(device_id);
        } else {
            self.query.filter = Some(TerminalCheckoutQueryFilter {
                created_at: None,
                device_id: Some(device_id),
                status: None
            })
        };

        self
    }

    pub fn status(mut self, status: TerminalCheckoutStatus) -> Self {
        if let Some(filter) = self.query.filter.as_mut() {
            filter.status = Some(status);
        } else {
            self.query.filter = Some(TerminalCheckoutQueryFilter {
                created_at: None,
                device_id: None,
                status: Some(status)
            })
        };

        self
    }

    pub fn into_builder(self) -> SearchTerminalCheckoutBodyBuilder {
        self.builder.query(self.query)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct CreateTerminalRefundBody {
    idempotency_key: String,
    refund: TerminalRefund,
}

#[derive(Default)]
pub struct CreateTerminalRefundBodyBuilder {
    refund: TerminalRefund,
}

impl CreateTerminalRefundBodyBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn amount_money(mut self, amount_money: Money) -> Self {
        self.refund.amount_money = Some(amount_money);

        self
    }

    pub fn device_id(mut self, device_id: String) -> Self {
        self.refund.device_id = Some(device_id);

        self
    }

    pub fn payment_id(mut self, payment_id: String) -> Self {
        self.refund.payment_id = Some(payment_id);

        self
    }

    pub fn reason(mut self, reason: String) -> Self {
        self.refund.reason = Some(reason);

        self
    }

    pub fn deadline_duration(mut self, reason: String) -> Self {
        self.refund.deadline_duration = Some(reason);

        self
    }

    pub async fn build(self)
        -> Result<CreateTerminalRefundBody, CreateTerminalRefundBodyBuildError> {
        if self.refund.device_id.is_none() ||
            self.refund.amount_money.is_none() ||
            self.refund.reason.is_none() ||
            self.refund.payment_id.is_none() {
            Err(CreateTerminalRefundBodyBuildError)
        } else {
            Ok(CreateTerminalRefundBody {
                idempotency_key: Uuid::new_v4().to_string(),
                refund: self.refund
            })
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SearchTerminalRefundBody {
    cursor: Option<String>,
    limit: Option<i32>,
    query: Option<TerminalRefundQuery>,
}

#[derive(Default)]
pub struct SearchTerminalRefundBodyBuilder {
    body: SearchTerminalRefundBody,
}

impl SearchTerminalRefundBodyBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn query(mut self, query: TerminalRefundQuery) -> Self {
        self.body.query = Some(query);

        self
    }

    pub fn query_builder(self) -> TerminalRefundQueryBuilder {
        TerminalRefundQueryBuilder::new(self)
    }

    pub fn limit(mut self, limit: i32) -> Self {
        self.body.limit = Some(limit);

        self
    }

    pub fn cursor(mut self, cursor: String) -> Self {
        self.body.cursor = Some(cursor);

        self
    }

    pub async fn build(self) -> SearchTerminalRefundBody {
        self.body
    }
}

pub struct TerminalRefundQueryBuilder {
    query: TerminalRefundQuery,
    builder: SearchTerminalRefundBodyBuilder,
}

impl TerminalRefundQueryBuilder {
    pub fn new(builder: SearchTerminalRefundBodyBuilder) -> Self {
        TerminalRefundQueryBuilder {
            query: Default::default(),
            builder
        }
    }

    pub fn created_at(mut self, created_at: TimeRange) -> Self {
        if let Some(filter) = self.query.filter.as_mut() {
            filter.created_at = Some(created_at)
        } else {
            self.query.filter = Some(TerminalRefundQueryFilter {
                created_at: Some(created_at),
                device_id: None,
                status: None
            })
        }

        self
    }

    pub fn device_id(mut self, device_id: String) -> Self {
        if let Some(filter) = self.query.filter.as_mut() {
            filter.device_id = Some(device_id)
        } else {
            self.query.filter = Some(TerminalRefundQueryFilter {
                created_at: None,
                device_id: Some(device_id),
                status: None
            })
        }

        self
    }

    pub fn pending(mut self) -> Self {
        if let Some(filter) = self.query.filter.as_mut() {
            filter.status = Some(TerminalCheckoutStatus::Pending)
        } else {
            self.query.filter = Some(TerminalRefundQueryFilter {
                created_at: None,
                device_id: None,
                status: Some(TerminalCheckoutStatus::Pending)
            })
        }

        self
    }

    pub fn in_progress(mut self) -> Self {
        if let Some(filter) = self.query.filter.as_mut() {
            filter.status = Some(TerminalCheckoutStatus::InProgress)
        } else {
            self.query.filter = Some(TerminalRefundQueryFilter {
                created_at: None,
                device_id: None,
                status: Some(TerminalCheckoutStatus::InProgress)
            })
        }

        self
    }

    pub fn cancel_requested(mut self) -> Self {
        if let Some(filter) = self.query.filter.as_mut() {
            filter.status = Some(TerminalCheckoutStatus::CancelRequested)
        } else {
            self.query.filter = Some(TerminalRefundQueryFilter {
                created_at: None,
                device_id: None,
                status: Some(TerminalCheckoutStatus::CancelRequested)
            })
        }

        self
    }

    pub fn canceled(mut self) -> Self {
        if let Some(filter) = self.query.filter.as_mut() {
            filter.status = Some(TerminalCheckoutStatus::Canceled)
        } else {
            self.query.filter = Some(TerminalRefundQueryFilter {
                created_at: None,
                device_id: None,
                status: Some(TerminalCheckoutStatus::Canceled)
            })
        }

        self
    }

    pub fn completed(mut self) -> Self {
        if let Some(filter) = self.query.filter.as_mut() {
            filter.status = Some(TerminalCheckoutStatus::Completed)
        } else {
            self.query.filter = Some(TerminalRefundQueryFilter {
                created_at: None,
                device_id: None,
                status: Some(TerminalCheckoutStatus::Completed)
            })
        }

        self
    }

    pub fn sort_ascending(mut self) -> Self {
        self.query.sort = Some(TerminalCheckoutQuerySort{ sort_order: Some(SortOrder::Asc) });

        self
    }

    pub fn sort_descending(mut self) -> Self {
        self.query.sort = Some(TerminalCheckoutQuerySort{ sort_order: Some(SortOrder::Desc) });

        self
    }

    pub fn into_builder(self) -> SearchTerminalRefundBodyBuilder {
        self.builder.query(self.query)
    }
}



#[cfg(test)]
mod test_terminals {
    use super::*;
    use crate::objects::enums::{Currency, SortOrder};
    use crate::objects::{TerminalCheckoutQueryFilter, TerminalCheckoutQuerySort};
    use crate::objects::enums::InventoryState::SoldOnline;

    #[actix_rt::test]
    async fn test_create_terminal_checkout_body_builder() {
        let expected = CreateTerminalCheckoutBody {
            idempotency_key: "".to_string(),
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

        let mut actual = CreateTerminalCheckoutBodyBuilder::new()
            .amount_money(Money { amount: Some(10), currency: Currency::USD })
            .device_options_builder()
            .device_id("some_id".to_string())
            .collect_signature()
            .skip_receipt_screen()
            .into_builder().unwrap()
            .build()
            .await.unwrap();

        actual.idempotency_key = "".to_string();

        assert_eq!(format!("{:?}", expected), format!("{:?}", actual))
    }

    #[actix_rt::test]
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

        let actual = SearchTerminalCheckoutBodyBuilder::new()
            .query_builder()
            .sort_ascending()
            .device_id("some_id".to_string())
            .into_builder()
            .limit(10)
            .build()
            .await;

        assert_eq!(format!("{:?}", expected), format!("{:?}", actual))
    }

    #[actix_rt::test]
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

        assert!(res.is_ok())
    }

    #[actix_rt::test]
    async fn test_create_terminal_refund_body_builder() {
        let expected = CreateTerminalRefundBody {
            idempotency_key: "".to_string(),
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

        let mut actual = CreateTerminalRefundBodyBuilder::new()
            .payment_id("some_id".to_string())
            .device_id("some_id".to_string())
            .reason("some reason".to_string())
            .amount_money(Money { amount: Some(10), currency: Currency::USD })
            .build()
            .await.unwrap();

        actual.idempotency_key = "".to_string();

        assert_eq!(format!("{:?}", expected), format!("{:?}", actual))
    }

    #[actix_rt::test]
    async fn test_create_terminal_refund_body_builder_fail() {

        let res = CreateTerminalRefundBodyBuilder::new()
            .payment_id("some_id".to_string())
            .device_id("some_id".to_string())
            .amount_money(Money { amount: Some(10), currency: Currency::USD })
            .build()
            .await;

        assert!(res.is_err())
    }

    #[actix_rt::test]
    async fn test_search_terminal_refund_body_builder() {
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

        let actual = SearchTerminalRefundBodyBuilder::new()
            .limit(10)
            .cursor("some cursor".to_string())
            .query_builder()
            .device_id("some_id".to_string())
            .sort_descending()
            .cancel_requested()
            .into_builder()
            .build()
            .await;

        assert_eq!(format!("{:?}", expected), format!("{:?}", actual))
    }
}

