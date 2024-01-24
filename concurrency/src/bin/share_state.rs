use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let m = Mutex::new(5); // mutual exclusion, allows only on ethread to access some data at any
                           // given time

    {
        let mut num = m.lock().unwrap(); // block the current thread
        *num = 6;
    }

    println!("m = {:?}", m);
    // ----------------------

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter); // use atomics to share across threads.
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

