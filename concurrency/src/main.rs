use std::thread;
use std::time::Duration;

fn main() {
    interleaved_example();

    println!("=================================");

    transferring_ownership_example();
}

fn interleaved_example() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    // Need this to prevent exiting prematurely and terminating the spawned
    // thread before it has been able to print all of its statements.
    handle.join().unwrap();
}

fn transferring_ownership_example() {
    let v = vec![1, 2, 3, 4, 5];

    // Transfer ownership of values it uses (i.e. v) with move keyword
    let handle = thread::spawn(move || {
        println!("Here is a vector: {v:?}!");
    });

    handle.join().unwrap()
}
