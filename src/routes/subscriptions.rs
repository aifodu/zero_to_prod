use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SubscriptionFormData {
    email: String,
    name: String,
}

pub async fn subscribe(_form: web::Form<SubscriptionFormData>) -> impl Responder {
    HttpResponse::Ok().finish()
}
