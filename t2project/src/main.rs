// #[macro_use]
// extern crate rocket;

// #[get("/")]
// fn index() -> &'static str {
//     "okssokasksak hehehwe"
// }

// #[launch]
// fn rocket() -> _ {
//     rocket::build().mount("/", routes![index])
// }

fn main() {
    // let mut a:String = String::from("test string");
    // println!("{}",a);
    // fn changeA(&mut valText) {
    //     *valText = "teste".to_string();
    // }
    // println!("{}",a);
    let mut x: i32 = 0;

    while x <= 9999 {
        println!("{}", x);
        x = x + 1;
    }
}
