use std::sync::{Mutex, MutexGuard};

use actix_web::{
    App, HttpServer, Responder, Result, get, post,
    web::{Data, Json},
};
use rand::{RngCore, rngs::ThreadRng};
use serde::Deserialize;
use utoipa::{OpenApi, ToSchema};
use utoipa_actix_web::AppExt;
use utoipa_swagger_ui::SwaggerUi;

const pay_table: [f64; 20] = [
    0.2, 0.4, 0.5, 0.7, 1.0, 1.2, 1.4, 1.7, 2.1, 2.6, 3.3, 3.8, 4.6, 5.2, 6.7, 7.3, 8.8, 10.5,
    14.0, 20.0,
];

#[derive(Deserialize, ToSchema)]
struct SpinRequest {
    bet: f64,
}

#[derive(Deserialize, ToSchema)]
struct DepositRequest {
    amount: f64,
}

#[utoipa::path(responses((status = OK, body = String)))]
#[get("/balance")]
async fn balance(data: Data<Mutex<f64>>) -> impl Responder {
    let amount = data.lock().unwrap();
    amount.to_string()
}

#[utoipa::path(responses((status = OK, body = String)))]
#[post("/deposit")]
async fn deposit(request: Json<DepositRequest>, data: Data<Mutex<f64>>) -> impl Responder {
    let mut amount: MutexGuard<'_, f64> = data.lock().unwrap();
    *amount += request.amount;
    amount.to_string()
}

#[utoipa::path(responses((status = OK, body = String)))]
#[post("/spin")]
async fn spin(request: Json<SpinRequest>, data: Data<Mutex<f64>>) -> Result<impl Responder> {
    let mut amount: MutexGuard<'_, f64> = data.lock().unwrap();
    if amount.lt(&request.bet) {
        Err(actix_web::error::ErrorPaymentRequired("balance too low"))
    } else {
        *amount -= request.bet;

        let mut rng: ThreadRng = rand::rng();
        let n: u32 = rng.next_u32() % 100;
        if n >= 20 {
            Err(actix_web::error::ErrorImATeapot("better luck next time"))
        } else {
            let winnings: f64 = request.bet * pay_table[n as usize];
            *amount += winnings;
            Ok(winnings.to_string())
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[derive(OpenApi)]
    #[openapi(
        tags(
            (name = "Wallet", description = "Wallet API.")
        )
    )]
    struct ApiDoc;

    let balance_amount: Data<Mutex<f64>> = Data::new(Mutex::new(0.0));

    HttpServer::new(move || {
        App::new()
            .into_utoipa_app()
            .openapi(ApiDoc::openapi())
            .service(balance)
            .service(deposit)
            .service(spin)
            .app_data(balance_amount.clone())
            .openapi_service(|api| {
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", api)
            })
            .into_app()
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
