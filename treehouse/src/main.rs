use std::io::stdin;

struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(n: &str, g: &str) -> Self {
        Self {
            name: n.to_lowercase(),
            greeting: g.to_string(),
        }
    }

    fn greet(&self) {
        println!("{}", self.greeting);
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
    println!("Hello what is your name?");
    let name = whats_ur_name();

    let the_list = [
        Visitor::new("robert", "how u doing Robert"), 
        Visitor::new("dave", "Good to see you dave"), 
        Visitor::new("frankie", "Who invited Frankie! Get in here you crazy"),
    ];

    let known = the_list.iter().find(|v| v.name == name);

    match known {
        Some(v) => v.greet(),
        None => print!("You are not on the visitor list, get out!")
    }
}
