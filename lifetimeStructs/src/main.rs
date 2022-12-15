#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_assignments)]
#[allow(unused_parens)]

struct Square {
    width: u32,
    height: u32,
}

impl Square {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn getWidth(&self) -> u32 {
        self.width
    }
    fn getHeight(&self) -> u32 {
        self.height
    }
    fn change_value(&mut self, new_width: u32) {
        self.width = new_width;
    }
}

fn main() {
    // let mut square = Square {
    //     width: 30,
    //     height: 45,
    // };
    // square.change_value(21);
    // println!("{}", square.area());
    // println!("{}", square.getHeight());
    // println!("{}", square.getWidth());

    println!("{:?}", "oi".to_string().as_bytes());
}
