use std::io::stdin;

#[derive(Debug)]
struct Visitor {
    name: String,
    greeting: String,
    action: VisitorAction,
    age: i8,
}

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

impl Visitor {
    fn new(name: &str, greeting: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
            action,
            age,
        }
    }

    fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => println!("{}", self.greeting),
            VisitorAction::AcceptWithNote { note } => {
                println!("{}, note: {}", self.greeting, note);
                if self.age < 21 {
                    println!("Do not serve alcohol to {}.", self.name);
                }
            }
            VisitorAction::Refuse => println!("Sorry {}, you are not allowed here.", self.name),
            VisitorAction::Probation => println!(
                "{}, you are a probationary member. Please follow the rules.",
                self.name
            ),
        }
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
    let mut visitor_list = vec![
        Visitor::new(
            "alice",
            "hello alice, welcome back!",
            VisitorAction::Accept,
            30,
        ),
        Visitor::new(
            "bob",
            "hi bob, good to see you again!",
            VisitorAction::AcceptWithNote {
                note: String::from("underage"),
            },
            2,
        ),
        Visitor::new(
            "carol",
            "hey carol, nice to see you!",
            VisitorAction::Refuse,
            28,
        ),
        Visitor::new(
            "dave",
            "welcome dave, make yourself at home!",
            VisitorAction::Probation,
            35,
        ),
    ];

    loop {
        println!("Hello, what's your name? (leave empty and press enter to quit)");
        let name = what_is_your_name();
        let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);

        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if name.is_empty() {
                    break;
                } else {
                    println!(
                        "{} is not on the visitor list. You're on probation for now, {}.",
                        name, name
                    );
                    visitor_list.push(Visitor::new(
                        &name,
                        "new friend",
                        VisitorAction::Probation,
                        0,
                    ));
                }
            }
        }
    }

    println!("the final list of visitors:");
    println!("{:#?}", visitor_list);
}
