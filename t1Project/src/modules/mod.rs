use owo_colors::OwoColorize;

pub fn plusfn(a: i32, b: i32) {
    let mut sum: i32 = a + b;
    sum += 10;
    println!("{}", sum.bright_yellow());
}

pub fn vecTut(a: i32, b: i32) {
    let vec1: Vec<i32> = vec![a, b];
    println!("{:?} - {} ", vec1.red(), vec1.capacity().green());
    let mut vec2: Vec<i32> = Vec::<i32>::with_capacity(5);
    vec2 = (0..5).collect();
    println!("{:?} - {}", vec2.red(), vec2.capacity().yellow());
    let mut vec3: Vec<String> = Vec::<String>::with_capacity(5);
    vec3.push("vfkd".to_string());
    vec3.push("vd".to_string());
    vec3.push("asaedbhj".to_string());
    vec3.reverse();
    println!("{:?}", vec3.green());
}

pub fn strTut(sta: String, stb: String) {
    println!("{} - {}", sta.bright_cyan(), stb.bright_green());
}

pub fn retSomething(retA: String) -> String {
    return retA;
}