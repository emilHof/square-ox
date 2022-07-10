/*!
Payment functionality of the [Square API](https://developer.squareup.com).
*/

use crate::client::SquareClient;
use crate::api::{Verb, SquareAPI};
use crate::errors::PaymentBuildError;
use crate::errors::SquareError;
use crate::objects::{enums::Currency, Money};
use crate::response::SquareResponse;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    /// Create a payment with the given [Payment](Payment) to the Square API
    /// and get the response back
    ///
    /// # Arguments
    /// * `payment` - A [Payment](Payment) created from the [PaymentBuilder](PaymentBuilder)
    pub async fn create(self, payment: PaymentRequest) -> Result<SquareResponse, SquareError> {
        self.client.request(
            Verb::POST,
            SquareAPI::Payments,
            Some(&payment),
            None,
        ).await
    }

}

/// The representation of a payment to the square API
/// containing a minimal set of fields for a payment
/// to be successfully processed.
#[derive(Serialize, Debug, Deserialize)]
pub struct PaymentRequest {
    #[serde(rename(serialize = "source_id"))]
    source_id: String,
    idempotency_key: String,
    amount_money: Money,
    #[serde(skip_serializing_if = "Option::is_none")]
    verification_token: Option<String>,
}

/// The [PaymentBuilder](PaymentBuilder)
pub struct PaymentBuilder {
    source_id: Option<String>,
    amount_money: Option<Money>,
    verification_token: Option<String>,
}

impl Default for PaymentBuilder {
    fn default() -> Self {
        Self {
            source_id: None,
            amount_money: None,
            verification_token: None,
        }
    }
}

impl PaymentBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn source_id(mut self, source_id: String) -> Self {
        self.source_id = Some(source_id);

        self
    }

    pub fn amount(mut self, amount: i64, currency: Currency) -> Self {
        self.amount_money = Some(Money { amount: Some(amount), currency });

        self
    }

    pub fn verification_token(mut self, token: String) -> Self {
        self.verification_token = Some(token);

        self
    }

    pub async fn build(&self) -> Result<PaymentRequest, PaymentBuildError> {
        let source_id = match &self.source_id {
            Some(n) => n.clone(),
            None => return Err(PaymentBuildError),
        };

        // The idempotency key just needs to be a random string
        // it is advised to use a v4 uuid by stripe
        let idempotency_key = Uuid::new_v4().to_string();

        let amount_money = match &self.amount_money {
            Some(n) => n.clone(),
            None => return Err(PaymentBuildError),
        };

        let verification_token = self.verification_token.clone();

        Ok(PaymentRequest {
            source_id,
            idempotency_key,
            amount_money,
            verification_token,
        })
    }
}

#[cfg(test)]
mod test_payments {
    use super::*;
    
    #[actix_rt::test]
    async fn test_create_payment() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);
        
        let input = PaymentRequest {
            source_id: "cnon:card-nonce-ok".to_string(),
            idempotency_key: Uuid::new_v4().to_string(),
            amount_money: Money { amount: Some(10), currency: Currency::USD },
            verification_token: None
        };

        let res = sut.payments()
            .create(input)
            .await;

        assert!(res.is_ok())
    }
}
