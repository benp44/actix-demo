mod arithmetic_service;

use actix::SyncArbiter;
use arithmetic_service::{ArithmeticService, Square};
use futures::{stream::FuturesUnordered, StreamExt};
use std::thread;

#[actix::main]
async fn main() {
    println!("Running main on thread: {:?}", thread::current().id());

    let processing_threads = 3;
    let service_address = SyncArbiter::start(processing_threads, || ArithmeticService);

    let mut futures = FuturesUnordered::new();
    futures.push(service_address.send(Square::new(5_i64)));
    futures.push(service_address.send(Square::new(2_i64)));
    futures.push(service_address.send(Square::new(3_i64)));

    while let Some(send_result) = futures.next().await {
        match send_result {
            Ok(square) => println!("Got square: {}", square),
            Err(err) => println!("Error communicating with service: {}", err),
        };
    }
}
