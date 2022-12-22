#![allow(dead_code)]

#[derive(Debug)]
struct City {
    name: String,
    population: u64,
}

// fn sort_pop(city: &mut Vec<City>) {
//     city.sort_by_key(pop_helper);
// }

// fn pop_helper(pop: &City) {
//     pop.population;
// }

fn sort_pop_closure(pop: &mut Vec<City>) {
    pop.sort_by_key(|p| p.population);
}

pub fn run_1() {
    println!("==CLOSURES==");
    let a = City {
        name: "A1-N".to_string(),
        population: 1000,
    };
    let b = City {
        name: "B1-Z".to_string(),
        population: 2000,
    };
    let c = City {
        name: "C1-X".to_string(),
        population: 3000,
    };

    let mut vec: Vec<City> = Vec::new();

    vec.push(a);
    vec.push(b);
    vec.push(c);

    sort_pop_closure(&mut vec);

    println!("{:?}", vec);
    println!("============");
}

pub fn run_2() {
    println!("==CLOSURES==");

    let add: _ = |n: i32, m: i32| n + m;

    println!("{}", add(5, 19));
    println!("============");
}
pub fn run_3() {
    println!("==CLOSURES==");
    let mut y = 10;
    let mut add: _ = |n: i32| {
        y += n;
        y
    };
    // let mut copy = add;
    println!("{}", add(5));
    println!("============");
}
