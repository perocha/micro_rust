use rouille;
use crate::api::Status;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct Response {
    message: String,
}

#[derive(Deserialize)]
struct Request {
    order_id: u16,
    description: String,
    order_types: Vec<String>,
}

pub fn serve(req: &rouille::Request) -> rouille::Response {
    match rouille::input::json_input::<Request>(req) {
        Ok(_) => {}
        _ => return rouille::Response::from(Status::BadRequest),
    };
    rouille::Response::json(&Response {
        message: String::from("Order created!"),
    })
}