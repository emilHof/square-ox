/*!
Payment functionality of the [Square API](https://developer.squareup.com).
*/

use crate::client::SquareClient;
use crate::api::{Verb, SquareAPI};
use crate::errors::{PaymentBuildError, ValidationError};
use crate::errors::SquareError;
use crate::objects::{Address, CashPaymentDetails, enums::Currency, ExternalPaymentDetails, Money, Payment};
use crate::response::SquareResponse;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::builder::{Builder, ParentBuilder, Validate};
use crate::objects::enums::SortOrder;

impl SquareClient {
    pub fn payments(&self) -> Payments {
        Payments {
            client: &self,
        }
    }
}

pub struct Payments<'a> {
    client: &'a SquareClient,
}

impl<'a> Payments<'a> {
    /// Retrieves a list of payments taken by the account making the request.
    /// [Open in API Reference](https://developer.squareup.com/reference/square/payments/list-payments)
    ///
    /// # Arguments
    /// * `parameters` - A vector of parameters created through the
    /// [ListPaymentsParametersBuilder](ListPaymentsParametersBuilder)
    pub async fn list(self, parameters: Option<Vec<(String, String)>>) -> Result<SquareResponse, SquareError> {
        self.client.request(
            Verb::GET,
            SquareAPI::Payments("".to_string()),
            None::<&PaymentRequest>,
            parameters,
        ).await
    }

    /// Create a payment with the given [Payment](Payment) to the Square API
    /// and get the response back
    ///
    /// # Arguments
    /// * `payment` - A [Payment](Payment)
    pub async fn create(self, payment: PaymentRequest) -> Result<SquareResponse, SquareError> {
        self.client.request(
            Verb::POST,
            SquareAPI::Payments("".to_string()),
            Some(&payment),
            None,
        ).await
    }

    /// Cancels (voids) a payment identified by the idempotency key that is specified in the request.
    /// [Open in API Reference](https://developer.squareup.com/reference/square/payments/cancel-payment-by-idempotency-key)
    ///
    /// # Arguments
    /// * `idempotency_key` - The idempotency key identifying the payment to be canceled.
    pub async fn cancel_by_idempotency_key(self, idempotency_key: String) -> Result<SquareResponse, SquareError> {
        self.client.request(
            Verb::POST,
            SquareAPI::Payments("/cancel".to_string()),
            Some(&CancelByIdempotencyKey { idempotency_key }),
            None,
        ).await
    }

    /// Retrieves details for a specific payment.
    /// [Open in API Reference](https://developer.squareup.com/reference/square/payments/get-payment)
    ///
    /// # Arguments
    /// * `payment_id` - The idempotency key identifying the payment to be canceled.
    pub async fn get(self, payment_id: String) -> Result<SquareResponse, SquareError> {
        self.client.request(
            Verb::GET,
            SquareAPI::Payments(format!("/{}", payment_id)),
            None::<&PaymentRequest>,
            None,
        ).await
    }

    /// Updates a payment with the APPROVED status.
    /// You can update the `amount_money` and `tip_money` using this endpoint.
    /// [Open in API Reference](https://developer.squareup.com/reference/square/payments/get-payment)
    ///
    /// # Arguments
    /// * `payment_id` - The idempotency key identifying the payment to be updated.
    /// * `body` - The request body with the updated [Payment](Payment) object.
    pub async fn update(self, payment_id: String, body: UpdatePaymentBody)
        -> Result<SquareResponse, SquareError> {
        self.client.request(
            Verb::PUT,
            SquareAPI::Payments(format!("/{}", payment_id)),
            Some(&body),
            None,
        ).await
    }

    /// Cancels (voids) a payment.
    /// [Open in API Reference](https://developer.squareup.com/reference/square/payments/cancel-payment)
    ///
    /// # Arguments
    /// * `payment_id` - The idempotency key identifying the payment to be canceled.
    pub async fn cancel(self, payment_id: String)
        -> Result<SquareResponse, SquareError> {
        self.client.request(
            Verb::POST,
            SquareAPI::Payments(format!("/{}/cancel", payment_id)),
            None::<&PaymentRequest>,
            None,
        ).await
    }

    /// Cancels (voids) a payment.
    /// [Open in API Reference](https://developer.squareup.com/reference/square/payments/cancel-payment)
    ///
    /// # Arguments
    /// * `payment_id` - The idempotency key identifying the payment to be completed.
    /// * `version_token` - Used for optimistic concurrency. This opaque token identifies the
    /// current [Payment](Payment) version that the caller expects. If the server has a different
    /// version of the [Payment](Payment), the update fails and a response with a VERSION_MISMATCH
    /// error is returned.
    pub async fn complete(self, payment_id: String, version_token: Option<String>)
        -> Result<SquareResponse, SquareError> {
        self.client.request(
            Verb::POST,
            SquareAPI::Payments(format!("/{}/complete", payment_id)),
            Some(&CompletePaymentBody {
                version_token
            }),
            None,
        ).await
    }
}

// -------------------------------------------------------------------------------------------------
// ListPaymentsParametersBuilder implementation
// -------------------------------------------------------------------------------------------------
#[derive(Default)]
pub struct ListPaymentsParametersBuilder {
    begin_time: Option<String>,
    end_time: Option<String>,
    sort_order: Option<SortOrder>,
    cursor: Option<String>,
    location_id: Option<String>,
    total: Option<i32>,
    last_4: Option<String>,
    card_brand: Option<String>,
    limit: Option<i32>,
}

impl ListPaymentsParametersBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    /// The timestamp for the beginning of the reporting period, in RFC 3339 format. Inclusive.
    /// Default: The current time minus one year.
    pub fn begin_time(mut self, begin_time: String) -> Self {
        self.begin_time = Some(begin_time);

        self
    }

    /// The timestamp for the end of the reporting period, in RFC 3339 format.
    // Default: The current time.
    pub fn end_time(mut self, end_time: String) -> Self {
        self.end_time = Some(end_time);

        self
    }

    /// The order in which results are listed.
    pub fn sort_ascending(mut self) -> Self {
        self.sort_order = Some(SortOrder::Asc);

        self
    }

    /// The order in which results are listed.
    pub fn sort_descending(mut self) -> Self {
        self.sort_order = Some(SortOrder::Desc);

        self
    }

    /// A pagination cursor returned by a previous call to this endpoint.
    /// Provide this cursor to retrieve the next set of results for the original query.
    pub fn cursor(mut self, cursor: String) -> Self {
        self.cursor = Some(cursor);

        self
    }

    /// Limit results to the location supplied. By default, results are returned for the default
    /// (main) location associated with the seller.
    pub fn location_id(mut self, location_id: String) -> Self {
        self.location_id = Some(location_id);

        self
    }

    /// The exact amount in the total_money for a payment.
    pub fn total(mut self, total: i32) -> Self {
        self.total = Some(total);

        self
    }

    /// The last four digits of a payment card.
    pub fn last_4(mut self, last_4: String) -> Self {
        self.last_4 = Some(last_4);

        self
    }

    /// The brand of the payment card (for example, VISA).
    pub fn card_brand(mut self, card_brand: String) -> Self {
        self.card_brand = Some(card_brand);

        self
    }

    /// The maximum number of results to be returned in a single page.
    /// It is possible to receive fewer results than the specified limit on a given page.
    //
    // The default value of 100 is also the maximum allowed value.
    // If the provided value is greater than 100, it is ignored and the default value is used
    // instead.
    pub fn limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);

        self
    }

    pub async fn build(self) -> Vec<(String,String)> {
        let ListPaymentsParametersBuilder {
            begin_time,
            end_time,
            sort_order,
            cursor,
            location_id,
            total,
            last_4,
            card_brand,
            limit,

        } = self;

        let mut res = vec![];


        if let Some(begin_time) = begin_time {
            res.push(("begin_time".to_string(), begin_time.to_string()))
        }
        if let Some(end_time) = end_time {
            res.push(("end_time".to_string(), end_time.to_string()))
        }
        if let Some(sort_order) = sort_order {
            res.push(("sort_order".to_string(), sort_order.to_string()))
        }
        if let Some(cursor) = cursor {
            res.push(("cursor".to_string(), cursor.to_string()))
        }
        if let Some(location_id) = location_id {
            res.push(("location_id".to_string(), location_id.to_string()))
        }
        if let Some(total) = total {
            res.push(("total".to_string(), total.to_string()))
        }
        if let Some(last_4) = last_4 {
            res.push(("last_4".to_string(), last_4.to_string()))
        }
        if let Some(card_brand) = card_brand {
            res.push(("card_brand".to_string(), card_brand.to_string()))
        }
        if let Some(limit) = limit {
            res.push(("limit".to_string(), limit.to_string()))
        }

        res
    }
}

// -------------------------------------------------------------------------------------------------
// PaymentRequest implementation
// -------------------------------------------------------------------------------------------------
/// The representation of a payment to the square API
#[derive(Serialize, Debug, Deserialize, Default)]
pub struct PaymentRequest {
    #[serde(rename(serialize = "source_id"), skip_serializing_if = "Option::is_none")]
    source_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    idempotency_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    amount_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accept_partial_authorization: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    app_fee_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    autocomplete: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    billing_address: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    buyer_email_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cash_details: Option<CashPaymentDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delay_action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delay_duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_details: Option<ExternalPaymentDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reference_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping_address: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    statement_description_identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    team_member_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tip_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    verification_token: Option<String>,
}

impl Validate for PaymentRequest {
    fn validate(mut self) -> Result<Self, ValidationError> where Self: Sized {
        if self.source_id.is_some() &&
            self.amount_money.is_some() {
            self.idempotency_key = Some(Uuid::new_v4().to_string());

            Ok(self)
        } else {
            Err(ValidationError)
        }
    }
}

impl<T: ParentBuilder> Builder<PaymentRequest, T> {
    pub fn source_id(mut self, source_id: String) -> Self {
        self.body.source_id = Some(source_id);

        self
    }

    pub fn amount(mut self, amount: i64, currency: Currency) -> Self {
        self.body.amount_money = Some(Money { amount: Some(amount), currency });

        self
    }

    pub fn verification_token(mut self, token: String) -> Self {
        self.body.verification_token = Some(token);

        self
    }
}

// -------------------------------------------------------------------------------------------------
// CancelByIdempotencyKey implementation
// -------------------------------------------------------------------------------------------------
#[derive(Serialize, Debug, Deserialize)]
struct CancelByIdempotencyKey {
    idempotency_key: String,
}

// -------------------------------------------------------------------------------------------------
// UpdatePaymentBody implementation
// -------------------------------------------------------------------------------------------------
#[derive(Serialize, Debug, Deserialize, Default)]
pub struct UpdatePaymentBody {
    idempotency_key: Option<String>,
    payment: Payment
}

impl Validate for UpdatePaymentBody {
    fn validate(mut self) -> Result<Self, ValidationError> where Self: Sized {
        self.idempotency_key = Some(Uuid::new_v4().to_string());

        Ok(self)
    }
}

impl<T: ParentBuilder> Builder<UpdatePaymentBody, T> {
    pub fn amount_money(mut self, amount_money: Money) -> Self {
        self.body.payment.amount_money = Some(amount_money);

        self
    }

    pub fn app_fee_money(mut self, app_fee_money: Money) -> Self {
        self.body.payment.app_fee_money = Some(app_fee_money);

        self
    }

    pub fn approved_money(mut self, approved_money: Money) -> Self {
        self.body.payment.approved_money = Some(approved_money);

        self
    }

    pub fn cash_details(mut self, cash_details: CashPaymentDetails) -> Self {
        self.body.payment.cash_details = Some(cash_details);

        self
    }

    pub fn tip_money(mut self, tip_money: Money) -> Self {
        self.body.payment.tip_money = Some(tip_money);

        self
    }

    pub fn version_token(mut self, version_token: String) -> Self {
        self.body.payment.version_token = Some(version_token);

        self
    }
}

#[derive(Serialize, Debug, Deserialize)]
struct CompletePaymentBody {
    version_token: Option<String>,
}

#[cfg(test)]
mod test_payments {
    use super::*;
    
    #[tokio::test]
    async fn test_create_payment() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);
        
        let input = PaymentRequest {
            source_id: Some("cnon:card-nonce-ok".to_string()),
            idempotency_key: Some(Uuid::new_v4().to_string()),
            amount_money: Some(Money { amount: Some(10), currency: Currency::USD }),
            accept_partial_authorization: None,
            app_fee_money: None,
            autocomplete: None,
            billing_address: None,
            buyer_email_address: None,
            cash_details: None,
            customer_id: None,
            delay_action: None,
            delay_duration: None,
            external_details: None,
            location_id: None,
            note: None,
            order_id: None,
            reference_id: None,
            shipping_address: None,
            statement_description_identifier: None,
            team_member_id: None,
            tip_money: None,
            verification_token: None
        };

        let res = sut.payments()
            .create(input)
            .await;

        assert!(res.is_ok())
    }

    #[tokio::test]
    async fn test_list_payments_parameters_builder() {
        let expected = vec![
            ("sort_order".to_string(), "ASC".to_string()),
            ("location_id".to_string(), "DMIOW91D2MDS".to_string()),
            ("total".to_string(), "10".to_string()),
            ("card_brand".to_string(), "Visa".to_string()),
        ];

        let actual = ListPaymentsParametersBuilder::new()
            .location_id("DMIOW91D2MDS".to_string())
            .card_brand("Visa".to_string())
            .total(10)
            .sort_ascending()
            .build()
            .await;

        assert_eq!(expected, actual);
    }

    #[tokio::test]
    async fn test_list_payments() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);

        let input = vec![
            ("sort_order".to_string(), "ASC".to_string()),
        ];

        let res = sut.payments()
            .list(Some(input))
            .await;

        assert!(res.is_ok())
    }

    // #[tokio::test]
    async fn test_cancel_by_idempotency_key() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);

        let res = sut.payments()
            .cancel_by_idempotency_key("d56c4e7f-c9f2-4204-b757-30abbcfae419".to_string())
            .await;

        assert!(res.is_ok())
    }

    #[tokio::test]
    async fn test_get_payment() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);

        let res = sut.payments()
            .get("tusWbiVmrQb2ibB06xqqRaVqKCXZY".to_string())
            .await;

        assert!(res.is_ok())
    }

    #[tokio::test]
    async fn test_update_payment_body_builder() {
        let expected = UpdatePaymentBody {
            idempotency_key: None,
            payment: Payment {
                id: None,
                amount_money: Some(Money { amount: Some(30), currency: Currency::USD }),
                app_fee_money: None,
                application_details: None,
                approved_money: None,
                bank_account_details: None,
                billing_address: None,
                buy_now_pay_later_details: None,
                buyer_email_address: None,
                capabilities: None,
                card_details: None,
                cash_details: None,
                created_at: None,
                customer_id: None,
                delay_action: None,
                delay_duration: None,
                delayed_until: None,
                device_details: None,
                external_details: None,
                location_id: None,
                note: None,
                order_id: None,
                processing_fee: None,
                receipt_number: None,
                receipt_url: None,
                reference_id: None,
                refund_ids: None,
                refunded_money: None,
                risk_evaluation: None,
                shipping_address: None,
                source_type: None,
                statement_description_identifier: None,
                status: None,
                team_member_id: None,
                tip_money: None,
                total_money: None,
                updated_at: None,
                version_token: None,
                wallet_details: None
            }
        };

        let mut actual = Builder::from(UpdatePaymentBody::default())
            .amount_money(Money { amount: Some(30), currency: Currency::USD })
            .build()
            .await
            .unwrap();

        assert!(actual.idempotency_key.is_some());

        actual.idempotency_key = None;

        assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
    }

    // #[tokio::test]
    async fn test_update_payment() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);

        let input = UpdatePaymentBody {
            idempotency_key: Some(Uuid::new_v4().to_string()),
            payment: Payment {
                id: None,
                amount_money: Some(Money { amount: Some(30), currency: Currency::USD }),
                app_fee_money: None,
                application_details: None,
                approved_money: None,
                bank_account_details: None,
                billing_address: None,
                buy_now_pay_later_details: None,
                buyer_email_address: None,
                capabilities: None,
                card_details: None,
                cash_details: None,
                created_at: None,
                customer_id: None,
                delay_action: None,
                delay_duration: None,
                delayed_until: None,
                device_details: None,
                external_details: None,
                location_id: None,
                note: None,
                order_id: None,
                processing_fee: None,
                receipt_number: None,
                receipt_url: None,
                reference_id: None,
                refund_ids: None,
                refunded_money: None,
                risk_evaluation: None,
                shipping_address: None,
                source_type: None,
                statement_description_identifier: None,
                status: None,
                team_member_id: None,
                tip_money: None,
                total_money: None,
                updated_at: None,
                version_token: None,
                wallet_details: None
            }
        };

        let res = sut.payments()
            .update("tusWbiVmrQb2ibB06xqqRaVqKCXZY".to_string(), input)
            .await;

        assert!(res.is_ok())
    }

    // #[tokio::test]
    async fn test_cancel_payment() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);


        let res = sut.payments()
            .cancel("tusWbiVmrQb2ibB06xqqRaVqKCXZY".to_string())
            .await;

        assert!(res.is_ok())
    }

    #[tokio::test]
    async fn test_complete_payment() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);


        let res = sut.payments()
            .complete("tusWbiVmrQb2ibB06xqqRaVqKCXZY".to_string(), None)
            .await;

        assert!(res.is_ok())
    }
}
