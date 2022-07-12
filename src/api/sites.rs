/*!
Sites functionality of the [Square API](https://developer.squareup.com).
 */

use crate::api::{SquareAPI, Verb};
use crate::client::SquareClient;
use crate::errors::SquareError;
use crate::response::SquareResponse;

impl SquareClient {
    pub fn sites(&self) -> Sites {
        Sites {
            client: &self,
        }
    }
}

pub struct Sites<'a> {
    client: &'a SquareClient,
}

impl<'a> Sites<'a> {
    /// Lists the Square Online sites that belong to a seller.
    /// [Open in API Reference](https://developer.squareup.com/reference/square/sites/list-sites)
    /// # Example: List all of the Square Online sites belonging to you
    /// ```rust
    ///use square_ox::{
    ///    response::{SquareResponse, ResponseError},
    ///    client::SquareClient
    ///    };
    ///
    /// async {
    ///     let sites = SquareClient::new("some_token")
    ///         .production()
    ///         .sites()
    ///         .list()
    ///         .await;
    /// };
    /// ```
    pub async fn list(self)
                      -> Result<SquareResponse, SquareError> {
        self.client.request(
            Verb::GET,
            SquareAPI::Sites("".to_string()),
            None::<&SquareResponse>,
            None,
        ).await
    }
}

#[cfg(test)]
mod test_sites {
    use super::*;

    #[actix_rt::test]
    async fn test_list_sites() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let access_token = env::var("SITE_ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let sut = SquareClient::new(&access_token).production();

        let res = sut.sites()
            .list()
            .await;

        assert!(res.is_ok())
    }
}