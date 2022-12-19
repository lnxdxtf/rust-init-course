trait Print {
    fn print_something(&self) {
        println!("alguma coisa- trait - caracteristica comum em struct?");
    }
    fn print_test(&self) -> String;
        
}

trait AnotherPrint {
    fn another_print_something(&self) {
        println!("print another print");
    }
    fn another_test(&self) -> String;
}
struct Test {
    a: String,
    b: String,
}

impl Print for Test{
    fn print_test(&self) -> String {
        format!("{}", self.a)
    }
}
impl AnotherPrint for Test{
    fn another_test(&self) -> String {
        format!("{}", self.b)
    }
}

//Imlement DROP - Automatic drop by rust

impl Drop for Test {
    fn drop(&mut self) {
        println!("Dropping this -- {}", self.a);
    }
}


// Just a trait
fn test_trait_parameter<T: Print>(item: &T) {
    println!("{} - Style-1",item.print_test());
}
fn test_trait_parameter_other_style(item: &impl Print) {
    println!("{} - Style-2",item.print_test());
}

// More styles

//error
// fn another_test_trait(item: &impl Print + AnotherPrint) {
//     println!("{} - Style-3",item.print_test());
// }
fn another_test_trait_other_style<T: Print + AnotherPrint>(item: &T) {
    println!("{} - Style-4",item.print_test());
}


pub fn main_traits() {
    let a = "a teste hehe".to_string();
    let b = "b testeb hoho".to_string();

    let test = Test{a: a, b: b};
    test.print_something();
    test_trait_parameter(&test);
    test_trait_parameter_other_style(&test);
    another_test_trait_other_style(&test);
}

// ----------------------------------------------------------------

use std::ops::Add;
#[derive(Debug)]
struct Point<T>{
    x: T,
    y: T,
}   

impl<T: std::ops::Add<Output = T>> Add for Point<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

pub fn main_overlaoding() {
    let p1 = Point{x: 1.2, y: 1.2};
    let p2 = Point{x:7.0, y:2.0};
    let p3_sum12 = p1 + p2;
    println!("{:?}", p3_sum12);
}

// ------------------------------------------------------------------------------------------------

pub fn printDebug<T: std::fmt::Debug>(value: T) {
    println!("{:?}", value);
}