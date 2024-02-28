use std::sync::mpsc; // can only have one receiver of messages but multiple producers.
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("Hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap(); // can panic, so we use unwrap to get the error if there is one.
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx2.send(val).unwrap(); // can panic, so we use unwrap to get the error if there is one.
        }
    });

    //let received = rx.recv().unwrap(); // recv blocks the main thread's execution while waiting for
    // value; can use try_recv if want to do other work while
    // waiting
    for received in rx {
        // treat rx as an iterator in this case
        println!("Got: {}", received);
    }
}

