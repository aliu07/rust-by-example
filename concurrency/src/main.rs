use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

fn main() {
    interleaved_example();
    println!("=================================");
    transferring_ownership_example();
    println!("=================================");
    channel_example();
    println!("=================================");
    sending_multiple_messages_example();
    println!("=================================");
    multiple_producers_example();
    println!("=================================");
    mutex_syntax_example();
    println!("=================================");
    sharing_mutex_example();
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

fn channel_example() {
    // mpsc = multiple producer, single consumer
    // tx = transmitter
    // rx = receiver
    let (tx, rx) = mpsc::channel();

    // Spawned thread needs to own 'tx', hence the move keyword
    thread::spawn(move || {
        let val = String::from("Hi!");
        println!("Sending '{val}' from spawned thread");
        // Ownership of 'val' is transferred when calling send().
        tx.send(val).unwrap();
        // Uncomment this line and you will realize it won't compile.
        // println!("val is {val}");
    });

    // recv() blocks the main thread's execution and waits until a value
    // is sent down the channel. Best if we need to wait to continue.
    //
    // try_recv() is another method that returns immediately a Result<T, E>.
    // Better suited if we can do other things while waiting for a value
    // to be received through the channel.
    let received = rx.recv().unwrap();
    println!("Got '{received}' in main thread");
}

fn sending_multiple_messages_example() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}

fn multiple_producers_example() {
    let (tx, rx) = mpsc::channel();

    // Clone the transmitter for first spawned thread
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Transfer ownership of original transmitter into 2nd spawned thread
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}

fn mutex_syntax_example() {
    let m = Mutex::new(5);

    {
        // lock() returns a smart ptr called MutexGuard
        let mut num = m.lock().unwrap();
        *num = 6;
    } // The smart ptr also has a Drop implementation that releases
    // the loack automatically when a MutexGuard goes out of scope

    println!("m = {m:?}");
}

fn sharing_mutex_example() {
    // 'counter' is reminescent of the interior mutability pattern.
    // 'counter' is immutable, but we could get a mutable reference
    // to the value inside it.
    // Similar to Rc where we can run into memory leaks, we can cause deadlocks
    // if we don't manage our mutexes well.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    // Spawn 10 threads
    for _ in 0..10 {
        // Can't use Rc here, but we can use Arc!
        // Arc = Atomic reference counter --> It's thread safe!
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
