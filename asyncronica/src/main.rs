use std::{pin::Pin, time::Duration};
use async_std::task;

use std::sync::Barrier;
use futures::join;

async fn hello() -> String {
    task::sleep(Duration::from_secs(2)).await;
    String::from("Hello")
}

async fn world() -> String {
    task::sleep(Duration::from_secs(1)).await;
    String::from(" World!")
}


async fn async_main() -> String {
    println!("Started!");
    let a = hello().await;
    let b = world().await;
    a + b.as_str()
}

fn main() {
    let a = Barrier::new(5);
    println!("{}", task::block_on(async_main()));
}