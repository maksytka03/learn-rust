use oop::{Button, Draw, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // draw select box
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(String::from("test")),
            Box::new(SelectBox {
                width: 100,
                height: 100,
                options: vec![String::from("yes"), String::from("no")],
            }),
            Box::new(Button {
                width: 100,
                height: 100,
                label: String::from("ok"),
            }),
        ],
    };

    screen.run()
}
