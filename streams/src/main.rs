#![allow(dead_code)]

use std::{pin::pin, time::Duration};
use trpl::{ReceiverStream, Stream, StreamExt};

fn main() {
    trpl::run(async {
        let messages = get_messages().timeout(Duration::from_millis(200));
        let intervals = get_intervals()
            .map(|count| format!("Interval: {count}"))
            .throttle(Duration::from_millis(100))
            .timeout(Duration::from_secs(10));
        let merged = messages.merge(intervals).take(20);
        let mut stream = pin!(merged);

        while let Some(res) = stream.next().await {
            match res {
                Ok(value) => {
                    println!("{value}");
                }
                Err(err) => {
                    println!("Problem: {err:?}");
                }
            }
        }
    })
}

fn stream_syntax_example() {
    trpl::run(async {
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = values.iter().map(|n| n * 2);
        let stream = trpl::stream_from_iter(iter);

        // Only keep values divisible by 3 or 5
        let mut filtered = stream.filter(|val| val % 3 == 0 || val % 5 == 0);

        while let Some(value) = filtered.next().await {
            println!("The value was: {value}");
        }
    });
}

fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];

        for (ix, msg) in messages.into_iter().enumerate() {
            let time_to_sleep = if ix % 2 == 0 { 100 } else { 300 };

            trpl::sleep(Duration::from_millis(time_to_sleep)).await;

            // Add error handling
            if let Err(send_error) = tx.send(format!("Message: '{msg}'")) {
                eprintln!("Cannot send message '{msg}': {send_error}");
                break;
            }
        }
    });

    ReceiverStream::new(rx)
}

fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let mut count = 0;

        loop {
            trpl::sleep(Duration::from_millis(1)).await;
            count += 1;

            if let Err(send_error) = tx.send(count) {
                eprintln!("Could not send interval {count}: {send_error}");
                break;
            };
        }
    });

    ReceiverStream::new(rx)
}
