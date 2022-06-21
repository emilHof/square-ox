/*!
The endpoints of the [Square API](https://developer.squareup.com).

To ensure the crate remains as extensible as possible, we are using
the Display trait for the URL of all of the endpoints
 */

pub mod payment;
pub mod bookings;
pub mod locations;
pub mod catalog;
pub mod customers;

use crate::client::ClientMode;
use crate::client::SquareClient;
use std::fmt;

/// All of the endpoints of the [Square API](https://developer.squareup.com)
/// for which we have implemented some of the functionality.
#[non_exhaustive]
pub enum SquareEndpoint {
    Payments,
    Bookings(String),
    Locations,
    Catalog(String),
    Customers(String),
}

/// All of the HTTP verbs that have been implemented and are accepted by the different
/// [Square API](https://developer.squareup.com) endpoints.
pub enum EndpointVerb {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

/// Implement the Display trait for all of the endpoints we need this allows
/// for them to be changed in the future without effecting the existing code
/// base.
impl fmt::Display for SquareEndpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SquareEndpoint::Payments => write!(f, "payments"),
            SquareEndpoint::Bookings(path) => write!(f, "bookings{}", path),  // TODO Implement Bookings
            SquareEndpoint::Locations => write!(f, "locations"),  // TODO Implement Locations
            SquareEndpoint::Catalog(path) => write!(f, "catalog{}", path),  // TODO Implement Catalog
            SquareEndpoint::Customers(path) => write!(f, "customers{}", path),  // TODO Implement Customers
        }
    }
}

impl SquareClient {
    pub fn endpoint(&self, end_point: SquareEndpoint) -> String {
        /// The main base URL for the Square API
        const SQUARE_PRODUCTION_BASE: &str = "https://connect.squareup.com/v2/";
        const SQUARE_SANDBOX_BASE: &str = "https://connect.squareupsandbox.com/v2/";

        match self.client_mode {
            ClientMode::Production => format!("{}{}", SQUARE_PRODUCTION_BASE, end_point),
            ClientMode::Sandboxed => format!("{}{}", SQUARE_SANDBOX_BASE, end_point),
        }
    }
}
