/*!
Catalog functionality of the [Square API](https://developer.squareup.com).
 */

use crate::client::SquareClient;
use crate::endpoint::{EndpointVerb, SquareEndpoint};
use crate::error::{};
use crate::error::SquareError;
use crate::response::{Response, ResponseError, SquareResponse};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::endpoint::bookings::{QueryFilter, SearchQuery};

impl SquareClient {
    async fn upsert_catalog_object(&self, object: CatalogObjectRequest)
        -> Result<SquareResponse, SquareError> {
        self.request(
            EndpointVerb::POST,
            SquareEndpoint::CatalogObject,
            Some(&object),
            None,
        ).await
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct CatalogObjectRequest {
    idempotency_key: Option<String>
}

#[derive(Default)]
pub struct CatalogObjectRequestBuilder {

}