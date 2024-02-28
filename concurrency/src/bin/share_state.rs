use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    //    let m = Mutex::new(5);
    //
    //    {
    //        let mut num = m.lock().unwrap(); // unwrap in case the lock cannot be acquired at the
    //                                         // moment (returns Option
    //        *num = 6;
    //    } // leaves the lock at the end of the scope
    //
    //    println!("{:?}", m);

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap(); // acquire lock to counter

            *num += 1; // increment counter
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

