fn main() {
    let name = String::from("Piegon");

    let bird1 = Bird{ name, attack: 12};    // since key and value are same for name

    bird1.bird_name();

}

struct Bird {
    name: String,
    attack: u64
}

impl Bird {
    fn bird_name(&self) {
        println!("{}", self.name);
    }
}