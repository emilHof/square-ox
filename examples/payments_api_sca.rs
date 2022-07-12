use square_ox::client::SquareClient;
use square_ox::errors::PaymentBuildError;
use square_ox::objects::enums::Currency;
use square_ox::api::payment::PaymentBuilder;

use actix_web::{middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::env;
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Enable logging
    env_logger::Builder::from_env(env_logger::Env::new().default_filter_or("debug"))
        .target(env_logger::Target::Stdout)
        .init();

    let port = "8080";
    let address = format!("0.0.0.0:{}", port);

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(process_payment)
            .data(AppState::init())
            // Serve the static files, to make testing easier.
            .service(actix_files::Files::new("/", "examples/static/").show_files_listing())
    })
    .bind(address)?
    .run()
    .await
}

struct AppState {
    client: SquareClient,
}

impl AppState {
    fn init() -> Self {
        dotenv().ok();
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        // let access_token = "EAAAEIQAdp294a9PHkqjSlWhi3qWBHnUwposv1n-TZrn0XW6xHGg810-72UrNr6U".to_string();
        let client = SquareClient::new(&access_token);

        Self { client }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentForm {
    location_id: String,
    source_id: String,
    amount: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    verification_token: Option<String>,
}

impl PaymentForm {
    fn get_price(&self) -> i64 {
        self.amount
            .chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>()
            .parse::<i64>()
            .unwrap_or(0)
    }
}

#[post("/process-payment")]
async fn process_payment(
    state: web::Data<AppState>,
    form: web::Json<PaymentForm>,
) -> impl Responder {
    // Get access to the square client
    let client = &state.client;

    // Serialize the square payment form from the request
    let payment_form = form.into_inner();

    // The amount must be given in the smallest denomination, convert to pence.
    let amount = payment_form.get_price() * 100;

    // Buld a payment using the infomation from the form
    let payment = PaymentBuilder::new()
        .amount(amount as i64, Currency::GBP)
        .source_id(payment_form.source_id);

    let payment = match payment_form.verification_token {
        Some(t) => payment.verification_token(t).build().await,
        None => payment.build().await,
    };

    let payment = match payment {
        Ok(p) => p,
        Err(e) => return HttpResponse::BadRequest().json(e),
    };

    // Create the payment and check the response
    match client.payments().create(payment).await {
        Ok(r) => HttpResponse::Ok()
            .set_header("Access-Control-Allow-Origin", "*")
            .json(r),
        Err(_) => HttpResponse::BadRequest().json(PaymentBuildError),
    }
}
