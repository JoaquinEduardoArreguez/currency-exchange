# Exchange Order Matchmaking
This project aims to create a simplified exchange order matchmaking system.  

Works by spawning two threads, one to generate random `Orders` and another one to find matches between the last created order and a pool containing all non-matched orders previously created.

This is an educational project I started while learning Rust.  

### Usage
After cloning this repository, you can run it using `cargo`
```sh
cd currency-exchange
cargo run
```
