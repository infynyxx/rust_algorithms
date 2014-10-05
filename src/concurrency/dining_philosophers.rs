use std::comm::{channel, Sender, Receiver};
use std::collections::HashSet;

struct Philosopher {
    name: String,
    pub next_philosopher: Option<Box<Philosopher>>,
    chopstick_sender: Sender<bool>,
    chopstick_receiver: Receiver<bool>
}
impl Philosopher {
    fn think(&self) {

    }

    fn eat(&self) {

    }

    fn get_chopsticks(&self) {

    }

    fn return_chopsticks(&self) {

    }

    fn dine(&self) {
        self.think();
        self.get_chopsticks();
        self.eat();
        self.return_chopsticks();
    }

    fn new(name: String) -> Philosopher {
        let (sender, receiver) = channel();
        Philosopher{
            name: name,
            chopstick_sender: sender,
            chopstick_receiver: receiver,
            next_philosopher: None
        }
    }
}

pub fn dining_philosopher() {
    let philosopher_names: HashSet<String> = ["Praj", "Sam", "Adam"]
        .iter()
        .map(|&x| x.to_string())
        .collect();
    let mut philosophers: Vec<Philosopher> = Vec::with_capacity(3);
    for name in philosopher_names.iter() {
        println!("Name: {}", name);
        philosophers.push(Philosopher::new(name.clone()));
    }
}
