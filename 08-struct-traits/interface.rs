fn main() {
    let name = String::from("Piegon");

    let bird1 = Bird{ name, attack: 12};    // since key and value are same for name

    bird1.bird_name();
    bird1.bird_attack();

    println!("can fly: {}, is_animal: {}", bird1.can_fly(), bird1. is_animal());
}

struct Bird {
    name: String,
    attack: u64
}

impl Bird {
    fn bird_name(&self) {
        println!("name: {}", self.name);
    }
    fn bird_attack(&self) {
        println!("attack: {}", self.attack);
    }
}

impl Animal for Bird {
    // implementing trait's method
    fn can_fly(&self) -> bool {
        true
    }   
    fn is_animal(&self) -> bool {
        false
    }
}

trait Animal {
    fn can_fly(&self) -> bool;
    fn is_animal(&self) -> bool {
        true
    }
}