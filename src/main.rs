#[derive(Debug)]
struct Chip {

}

impl Chip {
    pub fn new() -> Chip {
        Chip {

        }
    }
}

fn main() {
    println!("Hello, world!");
    let c = Chip::new();

    println!("{:?}", c);
}
