use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}")
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf parent = {}, weak = {}",
        Rc::strong_count(&leaf), // have ownership
        Rc::weak_count(&leaf)    // do not have ownership
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    //    let value = Rc::new(RefCell::new(5));
    //
    //    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    //
    //    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    //    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
    //
    //    *value.borrow_mut() += 10;
    //
    //    println!("a after = {:?}", a);
    //    println!("b after = {:?}", b);
    //    println!("c after = {:?}", c);
    //    let a = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil)))))));
    //    println!("Count after creating a = {:?}", Rc::strong_count(&a));
    //
    //    let b = Cons(3, Rc::clone(&a)); // another way to do this is a.clone()
    //    println!("Count after creating b = {:?}", Rc::strong_count(&a));
    //
    //    {
    //        let c = Cons(4, Rc::clone(&a));
    //        println!("Count after creating c = {:?}", Rc::strong_count(&a));
    //    }
    //    println!("Count after c goes out of scope = {}", Rc::strong_count(&a));
    //

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    //    let x = 5;
    //    let y = MyBox::new(x);
    //
    //    assert_eq!(5, x);
    //    assert_eq!(5, *y)
}

