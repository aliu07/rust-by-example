use std::pin::{Pin, pin};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    handling_multiple_futures_example();
    println!("===================================");
    // Msg passing w multiple producers
    message_passing_in_async_example();
    println!("===================================");
    using_join_all_on_futures_example();
    println!("===================================");
    join_macro_example();
    println!("===================================");
    racing_futures_example();
    println!("===================================");
    another_racing_example();
    println!("===================================");
    benchmark_sleep_vs_yield_now();
}

fn handling_multiple_futures_example() {
    trpl::run(async {
        let fut1 = async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let fut2 = async {
            for i in 1..5 {
                println!("hi number {i} from the second task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        trpl::join(fut1, fut2).await;
    });
}

fn message_passing_in_async_example() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        // Need to add move keyword so that tx goes out of scope and gets dropped,
        // thereby closing the channel and allowing the rx_fut async block to terminate.
        let tx1 = tx.clone();
        let tx1_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        let tx_fut = async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        };

        // trpl::join3(tx1_fut, tx_fut, rx_fut).await;
        // Instead of calling a diff join method for any arbitrary number of futures,
        // we can use a macro
        trpl::join!(tx1_fut, tx_fut, rx_fut);
    })
}

// When the number of futures is unknown, we can track them using a vector and
// join them all together like so:
fn using_join_all_on_futures_example() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        // Need to add move keyword so that tx goes out of scope and gets dropped,
        // thereby closing the channel and allowing the rx_fut async block to terminate.
        let tx1 = tx.clone();
        // Using pin! macro
        let tx1_fut = pin!(async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        let rx_fut = pin!(async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        });

        let tx_fut = pin!(async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        });

        // Using Box here requires heap allocation which equals more overhead.
        // We don't need heap allocation. The futures are local to this function.
        // Instead, we can use the pin! macro...
        //
        // let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> =
        //     vec![Box::pin(tx1_fut), Box::pin(rx_fut), Box::pin(tx_fut)];
        //
        // BETTER SOLUTION:
        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![tx1_fut, tx_fut, rx_fut];
        trpl::join_all(futures).await;
    })
}

fn join_macro_example() {
    trpl::run(async {
        // All async blocks have diff return types
        let a = async { 1u32 }; // Future<Output = u32>
        let b = async { "Hello!" }; // Future<Output = &str>
        let c = async { true }; // Future<Output = bool>

        let (a_result, b_result, c_result) = trpl::join!(a, b, c);
        println!("{a_result}, {b_result}, {c_result}");
    });
}

fn racing_futures_example() {
    trpl::run(async {
        let slow = async {
            println!("'slow' started.");
            trpl::sleep(Duration::from_millis(100)).await;
            println!("'slow' finished.");
        };

        let fast = async {
            println!("'fast' started.");
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'fast' finished.");
        };

        trpl::race(slow, fast).await;
    })
}

fn another_racing_example() {
    trpl::run(async {
        let one_ms = Duration::from_millis(1);

        let a = async {
            println!("'a' started.");
            slow("a", 30);
            trpl::sleep(one_ms).await; // Need to add these sleep statements to relinquish control of runtime
            slow("a", 10);
            trpl::sleep(one_ms).await; // and get interleaved output!
            slow("a", 20);
            trpl::sleep(one_ms).await;
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 75);
            trpl::sleep(one_ms).await;
            slow("b", 10);
            trpl::sleep(one_ms).await;
            slow("b", 15);
            trpl::sleep(one_ms).await;
            slow("b", 350);
            trpl::sleep(one_ms).await;
            println!("'b' finished.");
        };

        trpl::race(a, b).await;
    })
}

// Instead of sleeping, we can call yield_now() which conveys intent better
// and is also faster
fn benchmark_sleep_vs_yield_now() {
    trpl::run(async {
        let one_ns = Duration::from_nanos(1);
        let start = Instant::now();

        async {
            for _ in 1..1000 {
                trpl::sleep(one_ns).await;
            }
        }
        .await;

        let time = Instant::now() - start;
        println!(
            "'sleep' version finished after {} seconds.",
            time.as_secs_f32()
        );

        let start = Instant::now();
        async {
            for _ in 1..1000 {
                trpl::yield_now().await;
            }
        }
        .await;

        let time = Instant::now() - start;
        println!(
            "'yield' version finished after {} seconds.",
            time.as_secs_f32()
        );
    });
}

// HELPER FUNCTION
fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}
