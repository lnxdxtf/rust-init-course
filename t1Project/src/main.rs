#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_assignments)]
#[allow(unused_parens)]
mod modules;
fn main() {
    // let a: i32 = 10;
    // let b: i32 = 5;
    // modules::plusfn(a, b);
    // modules::vecTut(a, b);

    // let sta: String = String::from("Gabriel Gato Demais");
    // let stb: &str = "\x52\x75\x73\x74 - 🗿🍷";

    // modules::strTut(sta, stb.to_string());

    // println!("{}", modules::retSomething(stb.to_string()));

    // modules::retCondtionStr(stb.to_string());
    // let arr:Vec<i32> = [1,2,3,4,5,6,78,99,1672].to_vec();

    // modules::workWithForLoop(arr);

    let mut a: String = "teste".to_string();

    fn test_str(text: &mut String) {
        *text = text.to_uppercase();
    }
    println!("{}", a);
    test_str(&mut a);
    println!("{}", a);
}

//COPY CATEGORIES ( i32, f64, bool, char )
// let a:i32 = 10;
// let b = &a; - Reference to a when use (&{var}) - set pointer b to var a

// let a = String::from("olokinhomeu");
// let b = a; - wrong - x - move value from the var a to var b
// let b = &a; - correct - ok - reference to val a
// println!("{}",a);
// println!("{}",b);


//TO PRINT A VEC NEED ADD":?" in "{}" - EX:  println!("{:?}",vecx);