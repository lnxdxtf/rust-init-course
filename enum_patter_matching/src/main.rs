#[warn(dead_code)]

enum CodeType {
    NumId,
    StrId,
    MixId,
}
#[derive(Debug)]
enum DocOwner {
    PeopleId { code: String, name: String },
    CodeIdentification { idcode: String },
}

struct Car {
    model: String,
    brand: String,
    year: i32,
    code_type: CodeType,
    owner: Option<DocOwner>,
}

impl Car {
    fn get_data_owner(&self) -> String {
        format!(
            "{}",
            match &self.owner {
                Some(DocOwner::PeopleId { code, name }) => {
                    format!("code name")
                }
                Some(DocOwner::CodeIdentification { idcode }) => {
                    format!("id document")
                }
                None => {
                    format!("NONE MAN")
                }
            }
        )
    }
}

fn main() {
    // let my_car = Car {
    //     model: String::from("ferrari 503c"),
    //     brand: String::from("ferrari"),
    //     year: 2022,
    //     code_type: CodeType::NumId,
    //     owner: Some(DocOwner::PeopleId {
    //         code: "0845299303".to_string(),
    //         name: "Gabriel".to_string(),
    //     }),
    // };

    // let your_car = Car {
    //     model: String::from("ferrari 503c"),
    //     brand: String::from("ferrari"),
    //     year: 2022,
    //     code_type: CodeType::NumId,
    //     owner: None,
    // };
    // println!("{:?}", my_car.get_data_owner());
    // println!("{:?}", your_car.get_data_owner());

    // let x = 21;
    // match x {
    //     20 | 21 => println!("{:?}","20 or 21"),
    //     _ => println!("NONE")

    // }


//     let sh = Shape::Octagon;
//     print!("{}", sh.corners());

// }


// enum Shape {Triangle, Square, Pentagon, Octagon}
// impl Shape {
//     fn corners(&self) -> i8 {
//         match self {
//             Shape::Triangle => 3,
//             Shape::Square => 4,
//             Shape::Pentagon => 5,
//             Shape::Octagon => 8,
//             _ => 0,
//         }
//     }
// }