use square_rs::client::SquareClient;
use square_rs::error::SearchQueryBuildError;
use square_rs::endpoint::bookings::SearchQueryBuilder;

use actix_web::{middleware::Logger, post, get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::env;
use dotenv;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::new().default_filter_or("debug"))
        .target(env_logger::Target::Stdout)
        .init();

    dotenv::dotenv().ok();
    let port = env::var("PORT").expect("port to be set");
    let address = format!("0.0.0.0:{}", port);

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(list_availability)
            .service(list_locations)
            .data(AppState::init())
            .service(actix_files::Files::new("/", "examples/static/").show_files_listing())
    })
    .bind(address)?
    .run()
    .await
}

struct AppState {
    client: SquareClient,
    location_id: String,
}

impl AppState {
    fn init() -> Self {
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let location_id = env::var("LOCATION_ID").expect("LOCATION_ID to be set");
        let client = SquareClient::new(&access_token);

        Self { client, location_id }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryParams {
    start_at: String,
    end_at: String,
    segment_id: String,
}

#[post("/availability")]
async fn list_availability(
    state: web::Data<AppState>,
    form: web::Json<QueryParams>,
) -> impl Responder {
    println!("Availability query received!");

    let client = &state.client;
    let location_id = &state.location_id;

    let query_params = form.into_inner();

    let search_query = match SearchQueryBuilder::new()
        .location_id(location_id.clone())
        .start_at_range(query_params.start_at, query_params.end_at)
        .segment_filters(query_params.segment_id)
        .build()
        .await
    {
        Ok(p) => p,
        Err(e) => {
            println!("Failed to build search query!");
            return HttpResponse::BadRequest().json(e);
        }
    };

    match client.search_availability(search_query).await {
        Ok(r) => HttpResponse::Ok()
            .set_header("Access-Control-Allow-Origin", "*")
            .json(r),
        Err(_) => {
            println!("Failed to create payment");
            HttpResponse::BadRequest().json(SearchQueryBuildError)
        }
    }
}

#[get("/locations")]
async fn list_locations(
    state: web::Data<AppState>
) -> impl Responder {
    println!("Location query received!");

    let client = &state.client;

    match client.list_locations().await {
        Ok(r) => HttpResponse::Ok()
            .set_header("Access-Control-Allow-Origin", "*")
            .json(r),
        Err(_) => {
            println!("Failed to create payment");
            HttpResponse::BadRequest().finish()
        }
    }
}