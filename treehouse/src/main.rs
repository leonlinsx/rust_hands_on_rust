use std::io::stdin;

struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }

    fn greet_visitor(&self) {
        println!("{}", self.greeting);
    }
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name.trim().to_lowercase()
}

fn main() {
    println!("Hello, what's your name?");
    let name = what_is_your_name();
    println!("Hello, {}!", name);

    let visitor_list = [
        Visitor::new("alice", "hello alice, welcome back!"),
        Visitor::new("bob", "hi bob, good to see you again!"),
        Visitor::new("carol", "hey carol, nice to see you!"),
        Visitor::new("dave", "welcome dave, make yourself at home!"),
    ];
    let mut allow_them_in = false;

    for visitor in &visitor_list {
        if visitor == &name {
            allow_them_in = true;
        }
    }

    if allow_them_in {
        println!("Welcome to the treehouse, {}!", name);
    } else {
        println!("Sorry, {}. You are not on the visitor list.", name);
    }
}
