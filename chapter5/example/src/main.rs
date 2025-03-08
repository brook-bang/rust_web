#![warn(clippy::all)]

use std::any;

use handle_errors::return_error;
use warp::{filters::any::any, http::Method, Filter};

mod routes;
mod store;
mod types;

#[tokio::main]
async fn main() {
    let store = store::Store::new();
    let store_filer = warp:any().map(move || store.clone());
    let cors = warp.c
}