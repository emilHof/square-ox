/*!
The errors returned by components of the crate.
 */

use serde::{Deserialize, Serialize};
use crate::response::ResponseError;

#[derive(Serialize, Deserialize, Debug)]
pub struct SquareError(Option<Vec<ResponseError>>);

impl SquareError {
    pub fn from(response_errors: Option<Vec<ResponseError>>) -> Self {
        Self(response_errors)
    }

    pub fn get(self) -> Option<Vec<ResponseError>> {
        self.0
    }
}

impl From<reqwest::Error> for SquareError {
    fn from(r: reqwest::Error) -> Self {
        eprintln!("Reqwest Failed: {:?}", r);
        SquareError(None)
    }
}

impl From<reqwest::header::InvalidHeaderValue> for SquareError {
    fn from(r: reqwest::header::InvalidHeaderValue) -> Self {
        eprintln!("Reqwest Header Failed: {:?}", r);
        SquareError(None)
    }
}

impl From<serde_json::Error> for SquareError {
    fn from(s: serde_json::Error) -> Self {
        eprintln!("Serde JSON Failed: {:?}", s);
        SquareError(None)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentError {
    code: PaymentErrorCode,
    category: PaymentErrorCategory,
}

#[derive(Serialize, Deserialize, Debug)]
#[non_exhaustive]
pub enum PaymentErrorCode {
    // ADDRESS_VERIFICATION_FAILURE,
    // CARDHOLDER_INSUFFICIENT_PERMISSIONS,
    ErrorCode,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PaymentErrorCategory {
    PaymentErrorCat,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentBuildError;

#[derive(Serialize, Deserialize, Debug)]
pub struct LocationsBuildError;

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchQueryBuildError;

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomersListBuildError;

#[derive(Serialize, Deserialize, Debug)]
pub struct ListParametersBuilderError;

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomerBuildError;

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomerSearchQueryBuildError;

#[derive(Serialize, Deserialize, Debug)]
pub struct BookingsPostBuildError;

#[derive(Serialize, Deserialize, Debug)]
pub struct BookingsCancelBuildError;

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomerDeleteBuildError;

#[derive(Serialize, Deserialize, Debug)]
pub struct LocationBuildError;

#[derive(Serialize, Deserialize, Debug)]
pub struct CardBuildError;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateOrderRequestBuildError;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePaymentLinkBuildError;

#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentLinkBuildError;

#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectUpsertRequestBuildError;

#[derive(Serialize, Deserialize, Debug)]
pub struct InventoryChangeBodyBuildError;

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceOptionsBuildError;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTerminalCheckoutBodyBuildError;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTerminalRefundBodyBuildError;