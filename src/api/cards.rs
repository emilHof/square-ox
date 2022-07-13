/*!
Cards functionality of the [Square API](https://developer.squareup.com).
 */

use crate::client::SquareClient;
use crate::api::{Verb, SquareAPI};
use crate::errors::{CardBuildError, SquareError};
use crate::response::SquareResponse;
use crate::objects::{Address, Card};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::objects::enums::SortOrder;

impl SquareClient {
    pub fn cards(&self) -> Cards {
        Cards {
            client: &self,
        }
    }
}

pub struct Cards<'a> {
    client: &'a SquareClient,
}

impl<'a> Cards<'a> {
    /// Find a specific [Card](Card) by querying the [Square API](https://developer.squareup.com)
    /// with the card_id.
    /// # Arguments:
    /// * `card_id` - The id of the card you are looking to retrieve
    ///
    /// # Example
    /// ```rust
    ///use square_ox::{
    ///    response::{SquareResponse, ResponseError},
    ///    client::SquareClient
    ///    };
    ///
    /// async {
    ///     let locations = SquareClient::new("some_token")
    ///         .cards()
    ///         .retrieve("some_id".to_string())
    ///         .await;
    /// };
    pub async fn retrieve(self, card_id: String)
                               -> Result<SquareResponse, SquareError> {
        self.client.request(
            Verb::GET,
            SquareAPI::Cards(format!("/{}", card_id)),
            None::<&Card>,
            None,
        ).await
    }

    /// See which [Card](Card)s are on file by requesting the information from the
    /// [Square API](https://developer.squareup.com) and receiving them formatted as a
    /// list of [Card](Card)s.
    /// # Example
    /// ```rust
    ///use square_ox::{
    ///    response::{SquareResponse, ResponseError},
    ///    client::SquareClient
    ///    };
    ///
    /// async {
    ///     let locations = SquareClient::new("some_token")
    ///         .cards()
    ///         .list(None)
    ///         .await;
    /// };
    /// ```
    pub async fn list(self, search_query: Option<Vec<(String, String)>>)
                            -> Result<SquareResponse, SquareError> {
        self.client.request(
            Verb::GET,
            SquareAPI::Cards("".to_string()),
            None::<&Card>,
            search_query,
        ).await
    }

    /// Create a new [Card](Card) registered at the [Square API](https://developer.squareup.com).
    /// # Arguments:
    /// * `card` - A [Card](Card) wrapped in a [CardWrapper](CardWrapper)
    ///
    /// # Example
    /// ```rust
    ///use square_ox::{
    ///    response::{SquareResponse, ResponseError},
    ///    client::SquareClient
    ///    };
    /// use square_ox::api::cards::{CardBuilder, CardWrapper};
    ///
    /// async {
    ///     let card = CardBuilder::new()
    ///     .source_id("some_id".to_string())
    ///     .customer_id("some_id".to_string())
    ///     .build()
    ///     .await
    ///     .unwrap();
    ///
    ///     let locations = SquareClient::new("some_token")
    ///         .cards()
    ///         .create(card)
    ///         .await;
    /// };
    /// ```
    pub async fn create(self, card: CardWrapper)
                             -> Result<SquareResponse, SquareError> {
        self.client.request(
            Verb::POST,
            SquareAPI::Cards("".to_string()),
            Some(&card),
            None,
        ).await
    }

    /// Disable [Card](Card) at the [Square API](https://developer.squareup.com).
    /// # Arguments:
    /// * `card_id` - The id of the [Card](Card) you want to disable.
    ///
    /// # Example
    /// ```rust
    ///use square_ox::{
    ///    response::{SquareResponse, ResponseError},
    ///    client::SquareClient
    ///    };
    ///
    /// async {
    ///     let locations = SquareClient::new("some_token")
    ///         .cards()
    ///         .disable("some_id".to_string())
    ///         .await;
    /// };
    /// ```
    pub async fn disable(self, card_id: String)
                              -> Result<SquareResponse, SquareError> {
        self.client.request(
            Verb::POST,
            SquareAPI::Cards(format!("/{}/disable", card_id)),
            None::<&Card>,
            None,
        ).await
    }
}

#[derive(Default)]
pub struct ListCardsQueryBuilder {
    cursor: Option<String>,
    customer_id: Option<String>,
    include_disabled: Option<bool>,
    reference_id: Option<String>,
    sort_order: Option<SortOrder>,
}

impl ListCardsQueryBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn cursor(mut self, cursor: String) -> Self {
        self.cursor = Some(cursor);

        self
    }

    pub fn customer_id(mut self, customer_id: String) -> Self {
        self.customer_id = Some(customer_id);

        self
    }

    pub fn include_disabled(mut self) -> Self {
        self.include_disabled = Some(true);

        self
    }

    pub fn exclude_disabled(mut self) -> Self {
        self.include_disabled = Some(false);

        self
    }

    pub fn reference_id(mut self, reference_id: String) -> Self {
        self.reference_id = Some(reference_id);

        self
    }

    pub fn sort_ascending(mut self) -> Self {
        self.sort_order = Some(SortOrder::Asc);

        self
    }

    pub fn sort_descending(mut self) -> Self {
        self.sort_order = Some(SortOrder::Desc);

        self
    }

    pub async fn build(self) -> Vec<(String, String)> {
        let ListCardsQueryBuilder {
            cursor,
            customer_id,
            include_disabled,
            reference_id,
            sort_order,
        } = self;

        let mut res = vec![];

        if let Some(cursor) = cursor {
            res.push(("cursor".to_string(), cursor))
        }
        if let Some(customer_id) = customer_id {
            res.push(("customer_id".to_string(), customer_id))
        }
        if let Some(include_disabled) = include_disabled {
            res.push(("include_disabled".to_string(), include_disabled.to_string()))
        }
        if let Some(reference_id) = reference_id {
            res.push(("reference_id".to_string(), reference_id))
        }
        if let Some(sort_order) = sort_order {
            res.push(("sort_order".to_string(), sort_order.to_string()))
        }

        res
    }
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CardWrapper {
    pub(crate) card: Card,
    pub(crate) idempotency_key: String,
    pub(crate) source_id: String,
    pub(crate) verification_token: Option<String>,
}

#[derive(Default)]
pub struct CardBuilder {
    card: Card,
    source_id: Option<String>,
    verification_token: Option<String>,
}

impl CardBuilder {
    pub fn new() -> Self {
        Default::default()
    }
    
    pub fn customer_id(mut self, customer_id: String) -> Self {
        self.card.customer_id = Some(customer_id);
        
        self
    }
    
    pub fn billing_address(mut self, address: Address) -> Self {
        self.card.billing_address = Some(address);
        
        self
    }
    
    pub fn source_id(mut self, source_id: String) -> Self {
        self.source_id = Some(source_id);
        
        self
    } 
    
    pub async fn build(self) -> Result<CardWrapper, CardBuildError> {
        if self.source_id.is_none() || self.card.customer_id.is_none() {
            Err(CardBuildError)
        } else {
            Ok(
                CardWrapper {
                    card: self.card,
                    idempotency_key: Uuid::new_v4().to_string(),
                    source_id: self.source_id.unwrap(),
                    verification_token: self.verification_token
                }
            )
        }
    }
}

#[cfg(test)]
mod test_cards {
    use super::*;

    #[actix_rt::test]
    async fn test_list_cards_query_builder() {
        let expected = vec![
            ("cursor".to_string(), "dwcsdaw2390rec92".to_string()),
            ("include_disabled".to_string(), "false".to_string()),
            ("sort_order".to_string(), "ASC".to_string()),
        ];
        let actual = ListCardsQueryBuilder::new()
            .exclude_disabled()
            .sort_ascending()
            .cursor("dwcsdaw2390rec92".to_string())
            .build()
            .await;

        assert_eq!(expected, actual)
    }

    #[actix_rt::test]
    async fn test_list_cards() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);

        let input = vec![
            ("include_disabled".to_string(), "false".to_string()),
            ("sort_order".to_string(), "ASC".to_string()),
        ];

        let res = sut.cards()
            .list(Some(input))
            .await;

        assert!(res.is_ok())

    }

    #[actix_rt::test]
    async fn test_retrieve_card() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);

        let res = sut.cards()
            .retrieve("ccof:Es7R2xLyCWzmrKGI4GB".to_string())
            .await;

        assert!(res.is_ok())

    }

    #[actix_rt::test]
    async fn test_card_builder() {
        let expected = CardWrapper {
            card: Card {
                id: None,
                billing_address: None,
                bin: None,
                card_brand: None,
                card_co_brand: None,
                card_type: None,
                cardholder_name: None,
                customer_id: Some("EDH2RWZCFCRGZCZ99GMG8ZF59R".to_string()),
                enabled: None,
                exp_month: None,
                exp_year: None,
                fingerprint: None,
                last_4: None,
                merchant_id: None,
                prepaid_type: None,
                reference_id: None,
                version: None
            },
            idempotency_key: "".to_string(),
            source_id: "cnon:card-nonce-ok".to_string(),
            verification_token: None
        };

        let mut actual = CardBuilder::new()
            .customer_id("EDH2RWZCFCRGZCZ99GMG8ZF59R".to_string())
            .source_id("cnon:card-nonce-ok".to_string())
            .build()
            .await
            .unwrap();
        actual.idempotency_key = "".to_string();

        assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
    }

    // #[actix_rt::test]
    async fn test_create_card() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);

        let input = CardWrapper {
            card: Card {
                id: None,
                billing_address: None,
                bin: None,
                card_brand: None,
                card_co_brand: None,
                card_type: None,
                cardholder_name: None,
                customer_id: Some("EDH2RWZCFCRGZCZ99GMG8ZF59R".to_string()),
                enabled: None,
                exp_month: None,
                exp_year: None,
                fingerprint: None,
                last_4: None,
                merchant_id: None,
                prepaid_type: None,
                reference_id: None,
                version: None
            },
            idempotency_key: Uuid::new_v4().to_string(),
            source_id: "cnon:card-nonce-ok".to_string(),
            verification_token: None
        };

        let res = sut.cards()
            .create(input)
            .await;

        assert!(res.is_ok())
    }

    #[actix_rt::test]
    async fn test_disable_card() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token);

        let res = sut.cards()
            .disable("ccof:ce0ogxL3KIHfNd4Z4GB".to_string())
            .await;

        assert!(res.is_ok())

    }


}