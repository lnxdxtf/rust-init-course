#[cfg(test)]
#[test]
fn testing() {
    let result = 10 + 11;
    //assert  equals | if r == 21 return true else return false
    assert_eq!(result, 21);
}
#[test]
#[ignore] // ignore test
fn other_test() {
    let r = 10;
    assert_eq!(r, 10);
}
#[test]
fn some_test() {
    // If the function return true, the test is OK 
    assert!(some_test_call(4));
}

fn some_test_call<'a>(num: i32) -> bool {
    if num == 4 {
        return true;
    } else {
        return false;
    }
}
#[test]
fn test_not_equals() {
    let r = 212;
    //assert not equals | if r != 10 return true else return false
    assert_ne!(r, 10);
}

#[test]
#[should_panic] // the test will fail - but i know, so include this to run other tests
fn failed_test() {
    panic!("failed test, destiny 🔥");
}

//-----------------------
// TO RUN UNIQUE TEST BY FN NAME, USE:
// cargo test {fn_name}
//-----------------------

// fn main() {
//     println!("Hello, world!");
// }
