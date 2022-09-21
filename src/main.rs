mod domain;
mod repositories;
mod api;

#[macro_use]
extern crate rouille;
extern crate serde;

fn main() {
    println!("Starting http server");
    api::serve("0.0.0.0:8000");
}
