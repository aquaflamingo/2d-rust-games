use std::io::stdin;

#[derive(Debug)]
struct Visitor {
    name: String,
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
    fn new(n: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: n.to_lowercase(),
            action,
            age,
        }
    }

    fn greet(&self) {
        match &self.action {
            VisitorAction::Accept => println!("Welcome to da place {}", self.name),
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome to da place {}", self.name);
                println!("Hey get some {} for this dude", note);
                if self.age < 19 {
                    println!("No booze for this kid {} either!", self.name);
                }
            }
            VisitorAction::Probation => println!("Hey {} you are allowed in, but on probation", self.name),
            VisitorAction::Refuse => println!("You are not allowed in {}, get out of here!", self.name),
        }
    }
}


fn whats_ur_name() -> String {
    let mut name = String::new();

    stdin()
        .read_line(&mut name)
        .expect("Failed to read line"); 

    name
        .trim()
        .to_lowercase()
}

fn main() {
    let mut the_list = vec![
        Visitor::new("robert", VisitorAction::Accept, 21), 
        Visitor::new("dave", VisitorAction::AcceptWithNote { note: String::from("milk") }, 17), 
        Visitor::new("frankie", VisitorAction::Refuse, 30),
    ];

    loop {
        println!("Hello what is your name?");
        let name = whats_ur_name();

        let known = the_list.iter().find(|v| v.name == name);

        match known {
            Some(v) => v.greet(),
            None => {
                if name.is_empty() {
                    break;
                } else {
                    println!("{} is not on the visitor list, get out!", name);
                    the_list.push(Visitor::new(&name, VisitorAction::Probation, 0));
                }
            }
        }
    }

    println!("The final list of visitors:");
    println!("{:#?}", the_list);
}
