trait Gateway {
    fn getName(&self) -> String {
        String::from("Do not have any name!")
    }

    fn forward(&self) -> String {
        String::from("Can not forward!")
    }
}

struct Bus {
    name: String,
    link: String,
}

impl Bus {
    fn new(name: String, link: String) -> Self {
        Bus { name, link }
    }
}

impl Gateway for Bus {
    fn getName(&self) -> String {
        self.name.clone()
    }
}

fn main() {
    println!("Hello");
    let bus1 = Bus::new(String::from("GT1"), String::from("gg.com"));
    println!("Bus name = {}", bus1.getName());
    println!("Link name = {}", bus1.forward());
}
