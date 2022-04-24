use std::any::Any;

#[derive(PartialEq)]
enum PersonType {
    Human,
    BusinessMan,
}

trait Person {
    fn greet(&self) -> String;
    fn typ(&self) -> PersonType;
    fn as_any(&self) -> &dyn Any;
}

struct Human {
    name: String,
}

impl Human {
    fn breathe(&self) {
        println!("I'm breathing");
    }
}

impl Person for Human {
    fn greet(&self) -> String {
        format!("Hello, my name is {}!", self.name)
    }

    fn typ(&self) -> PersonType {
        PersonType::Human
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct BusinessMan {
    title: String,
    name: String,
}

impl BusinessMan {
    fn work(&self) {
        println!("I'm working");
    }
}

impl Person for BusinessMan {
    fn greet(&self) -> String {
        format!("Hello, my name is {} {}!", self.title, self.name)
    }

    fn typ(&self) -> PersonType {
        PersonType::BusinessMan
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn main() {
    let human = Human {
        name: "Fred".to_string(),
    };

    let businessman = BusinessMan {
        title: "Mr.".to_string(),
        name: "Bob".to_string(),
    };

    let mut people: Vec<Box<dyn Person>> = Vec::new();
    people.push(Box::new(human));
    people.push(Box::new(businessman));

    for person in people {
        println!("{}", person.greet());

        let maybe_human = person.as_any().downcast_ref::<Human>();
        if let Some(human) = maybe_human {
            human.breathe();
        }

        let maybe_businessman = person.as_any().downcast_ref::<BusinessMan>();
        if let Some(businessman) = maybe_businessman {
            businessman.work();
        }
    }
}
