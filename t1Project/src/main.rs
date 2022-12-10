#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_assignments)]
mod modules;
fn main() {
    let a: i32 = 10;
    let b: i32 = 5;
    modules::plusfn(a, b);
    modules::vecTut(a, b);

    let sta: String = String::from("Gabriel Gato Demais");
    let stb: &str = "\x52\x75\x73\x74 - 🗿🍷";

    modules::strTut(sta, stb.to_string());

    println!("{}", modules::retSomething(stb.to_string()));
}
