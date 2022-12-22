#![allow(dead_code)]
#![allow(non_snake_case)]

pub fn run_1() {
    let v1 = vec![1, 2, 3, 4, 5];
    for i in v1.iter() {
        print!("{} ", i);
    }
}

pub fn run_2() {
    let v2 = vec![1, 2, 3, 4, 5];
    let mut iter = (&v2).into_iter();
    while let Some(v) = iter.next() {
        print!("{}", v);
    }
}

pub fn run_3() {
    #[derive(Debug)]
    enum Currency {
        Real,
        Usd,
    }
    #[derive(Debug)]
    struct Product {
        name: String,
        value: f32,
        currency: Currency,
    }

    let mut productsVec: Vec<Product> = Vec::new();
    let quantity: i32 = 10;

    fn checkInventory(items: Vec<Product>, productName: String) -> Vec<Product> {
        items.into_iter().filter(|x| x.name == productName).collect()
    }

    for i in 0..quantity {
        productsVec.push(Product {
            name: format!("PRODUCT {}", i),
            value: 10.2,
            currency: Currency::Real,
        });
    }

    let foundedProduct = checkInventory(productsVec, "PRODUCT 1".to_string());
    println!("{:?}", foundedProduct.first());
}
