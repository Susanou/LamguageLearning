use std::io::stdin;

#[derive(Debug)]
struct Visitor{
    name: String,
    greeting: String,
}

impl Visitor {

    fn new(name: &str, greeting: &str) -> Self{
        Self{
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
        .expect("Failed to read libe");
    your_name.trim().to_lowercase()

}

fn main() {
    let mut visitor_list = vec![
        Visitor::new("bert", "Hello Bert, enjoy your stay here."),
        Visitor::new("steve", "Hi Steve. Your milk is in the fridge."),
        Visitor::new("fred", "Wow, who ivited Fred?"),
    ];

    loop {
        println!("What is your name?");
        let name = what_is_your_name();

        let known_visitor = visitor_list
                                .iter()
                                .find(|visitor| visitor.name == name);
        
        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if name.is_empty() {
                    break;
                } else {
                    println!("{} is not on the visitor list", name);
                    visitor_list.push(Visitor::new(&name, "New friend"));
                }
            }
        }
    }

    println!("The final list of visitors: ");
    println!("{:#?}", visitor_list);
    
}
