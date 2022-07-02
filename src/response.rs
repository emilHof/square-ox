/*!
This defines the possible responses you could receive from the
[Square API](https://developer.squareup.com).

The most of the structs have almost all of their fields set as optional as this makes dealing
with the [Square API](https://developer.squareup.com)'s response pattern more manageable.
 */

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
/// The [SquareResponse](SquareResponse) response defines the generic response type that encompasses
/// almost all possible [Square API](https://developer.squareup.com) responses. All fields are
/// optional to allow for handling of possible errors returned by the
/// [Square API](https://developer.squareup.com).
pub struct SquareResponse {
    #[serde(flatten)]
    pub response: crate::objects::Response,
    #[serde(default)]
    pub errors: Option<Vec<ResponseError>>,
    #[serde(default)]
    pub cursor: Option<String>,
    #[serde(default)]
    pub id_mapping: Option<Vec<(String, String)>>,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub cancelled_order_id: Option<String>,
}


/// The [ResponseError](ResponseError) defines the error schema returned by the
/// [Square API](https://developer.squareup.com) should an error occur. This makes error handling
/// possible by checking if the error field of the [SquareResponse](SquareResponse) is some.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResponseError {
    pub category: String,
    pub code: String,
    #[serde(default)]
    pub detail: Option<String>,
    #[serde(default)]
    pub field: Option<String>,
}