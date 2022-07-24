/*!
The [SquareClient](crate::client::SquareClient) allows the user of the crate
to use the [Square API](https://developer.squareup.com) in an idiomatic way.

# Example: Creating a client
In order to create a client you will need your account access token that can be found
in the [Developer Apps](https://developer.squareup.com/apps) page for the specific
application you are wanting to use.

```rust
 const ACCESS_TOKEN:&str = "your_square_access_token";

use square_ox::client::SquareClient;
let client = SquareClient::new(ACCESS_TOKEN);
```
After creating a client you will be able to use all of the clients methods.

*/
use crate::api::{SquareAPI, Verb};
use crate::errors::SquareError;
use crate::response::SquareResponse;

use reqwest::{header, Client};
use serde::Serialize;
use std::default::Default;

#[derive(Copy, Clone)]
pub enum ClientMode {
    Production,
    Sandboxed,
}

/// The default mode we start a client in is Sandboxed
impl Default for ClientMode {
    fn default() -> Self {
        Self::Sandboxed
    }
}

/// The SquareClient contains many useful methods allowing for convenient
/// use of the [Square API](https://developer.squareup.com).
#[derive(Clone)]
pub struct SquareClient {
    access_token: String,
    pub(crate) client_mode: ClientMode,
}

impl SquareClient {
    /// Create a new [SquareClient](SquareClient)
    ///
    /// # Arguments
    /// * `access_token` - The access token for the Square App you
    /// want to use the client with is required.
    ///
    /// # Example: Create a new client
    /// ```
    /// const ACCESS_TOKEN:&str = "your_square_access_token";
    /// use square_ox::client::SquareClient;
    ///
    /// let client = SquareClient::new(ACCESS_TOKEN);
    /// ```
    pub fn new(access_token: &str) -> Self {
        Self {
            access_token: access_token.to_string(),
            client_mode: Default::default(),
        }
    }

    /// Set the client to Production Mode
    ///
    /// # Arguments
    /// This method takes no arguments, as by default the client will use SandBox Mode.
    ///
    /// # Example
    /// ```
    /// const ACCESS_TOKEN:&str = "your_square_access_token";
    ///
    /// use square_ox::client::SquareClient;
    /// let client = SquareClient::new(ACCESS_TOKEN).production();
    /// ```
    pub fn production(self) -> Self {
        Self {
            access_token: self.access_token,
            client_mode: ClientMode::Production,
        }
    }

    /// Sends a request to a given [SquareAPI](crate::api::SquareAPI)
    /// # Arguments
    /// * `api` - The [SquareAPI](crate::api::SquareAPI) to send the request to
    /// * `body` - The json that will be included in the request.
    /// All types that meet the conditions to be deserialized to JSON are accepted.
    ///
    /// # Example:
    /// ```
    /// use env_logger::Builder;
    ///  async {
    ///     use square_ox::{
    ///         api::{
    ///             Verb, SquareAPI, payment::PaymentRequest
    ///         },
    ///         client,
    ///         builder::Builder
    ///     };
    ///     const ACCESS_TOKEN:&str = "your_square_access_token";
    ///     let payment = Builder::from(PaymentRequest::default()).build().await;
    ///
    ///     let client = client::SquareClient::new(ACCESS_TOKEN);
    ///     client.request( Verb::POST, SquareAPI::Payments("".to_string()), Some(&payment), None).await.expect("");
    /// };
    ///
    /// ```
    pub async fn request<T>(
        &self,
        verb: Verb,
        endpoint: SquareAPI,
        json: Option<&T>,
        parameters: Option<Vec<(String, String)>>,
    ) -> Result<SquareResponse, SquareError>
    where
        T: Serialize + ?Sized,
    {
        let url = self.endpoint(endpoint).clone();
        let authorization_header = format!("Bearer {}", &self.access_token);

        // Add the headers to the request
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&authorization_header)?,
        );

        // Create a client with the appropriate headers
        let client = Client::builder().default_headers(headers).build()?;

        println!("url: {}", &url);

        // Send the request to the Square API, and get the response
        let mut builder = match verb {
            Verb::GET => client.get(&url),
            Verb::POST => client.post(&url),
            Verb::PUT => client.put(&url),
            Verb::PATCH => client.patch(&url),
            Verb::DELETE => client.delete(&url),
        };

        // Add query parameters if there are any
        if let Some(parameters) = parameters {
            builder = builder.query(&parameters);
        }

        // Add a json body if there is one
        if let Some(json) = json {
            builder = builder.json(json)
        }

        // Deserialize the response into a SquareResponse
        // let response: SquareResponse = builder.send().await?.json().await?;

        // TODO remove the debug code!
        let response = builder.send().await?.text().await?;

        println!("{:?}", response);

        let response: SquareResponse = serde_json::from_str(&response)?;

        println!("{:?}", response);

        // handle the possibility of an error being returned by the Square API
        if response.errors.is_some() && response.errors.as_ref().unwrap().len() > 0 {
            return Err(SquareError::from(response.errors))
        }

        Ok(response)
    }
}
