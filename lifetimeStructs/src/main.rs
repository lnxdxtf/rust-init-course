#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_assignments)]
#[allow(unused_parens)]

struct Square {
    width: u32,
    height: u32,
}
//add methods -implementations methods
impl Square {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn get_width(&self) -> u32 {
        self.width
    }
    fn get_height(&self) -> u32 {
        self.height
    }
    fn change_value(&mut self, new_width: u32) {
        self.width = new_width;
    }
    // fn exec_all(&self) { //error cause change value - self needs to be &mut self
    fn exec_all(&mut self) {
        self.area();
        self.get_height();
        self.get_width();
        self.change_value(32);
    }
}

struct TimeInt<'a> {
    time: &'a i32,
}

// fn main() {
//     let mut square = Square {
//         width: 30,
//         height: 45,
//     };
//     square.exec_all();

//     let timeint: TimeInt = TimeInt { time: &10 };
//     println!("{}", timeint.time.to_string());
//     println!("{:?}", "oi, junin".to_string().as_bytes());
// }

struct Car{
    mpg: i32,
    color: String,
    top_speed: i32,
}

impl Car {
    fn set_mpg(&mut self, value: i32) {
        self.mpg = value;
    }
    fn set_color(&mut self, value: String) {
        self.color = value;
    }
    fn set_top_speed(&mut self, value: i32) {
        self.top_speed = value;
    }
    fn exec_all(&mut self) {
        self.set_color("white".to_string());
        self.set_mpg(50);
        self.set_top_speed(500);
    }
}
fn main() {
    let mut carCourses:Car = Car{color: "red".to_string(), mpg: 1, top_speed: 100};
    carCourses.exec_all();
}