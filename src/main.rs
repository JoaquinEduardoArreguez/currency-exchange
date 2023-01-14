mod matchmaking;
mod order;

use colored::*;
use order::Order;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;

use crate::order::Status;

const ORDER_CREATION_INTERVAL_MS: u64 = 1000;

fn main() {
    let (order_creator_tx, order_creator_rx) = mpsc::channel();

    let order_creation_handler = spawn_order_creator(order_creator_tx, ORDER_CREATION_INTERVAL_MS);
    let order_matchmaking_handler = spawn_order_matchmaker(order_creator_rx);

    match order_creation_handler.join() {
        Ok(ok) => println!("{:?}", ok),
        Err(error) => println!("{:?}", error),
    }
    match order_matchmaking_handler.join() {
        Ok(ok) => println!("{:?}", ok),
        Err(error) => println!("{:?}", error),
    }
}

fn spawn_order_creator(
    order_creator_tx: Sender<Order>,
    creation_pace: u64,
) -> thread::JoinHandle<()> {
    thread::spawn(move || loop {
        let random_order = order::generate_random();
        match order_creator_tx.send(random_order) {
            Ok(()) => {}
            Err(error) => panic!("Problem in send {:?}", error),
        }

        thread::sleep(Duration::from_millis(creation_pace));
    })
}

fn spawn_order_matchmaker(order_creator_rx: Receiver<Order>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut all_orders: Vec<Order> = vec![];
        let mut matching_orders = 0;
        for received in order_creator_rx {
            println!("Pending orders: {:?}", all_orders.len());
            println!("matched orders: {:?}", matching_orders);
            println!("processing: {:?}", received);

            if received.status == Status::Pending {
                match matchmaking::find_match(&received, &mut all_orders) {
                    Some(matching_tuple) => {
                        matching_orders = matching_orders + 1;

                        println!(
                            "{}",
                            ("matching tuple: \n".to_string()
                                + &matching_tuple.0.to_string()
                                + "\n"
                                + &matching_tuple.1.to_string())
                                .blue()
                        );

                        if let Some(matching_order_index) = all_orders
                            .iter()
                            .position(|order| *order.uuid == matching_tuple.1.uuid)
                        {
                            let removed_order = all_orders.swap_remove(matching_order_index);

                            println!(
                                "{}",
                                ("removed: ".to_string() + &removed_order.to_string()).red()
                            )
                        }
                    }
                    None => {
                        println!(
                            "{}",
                            ("added: ".to_string() + &received.to_string()).green()
                        );
                        all_orders.push(received);
                    }
                }
            }

            println!("===================================");
        }
    })
}
