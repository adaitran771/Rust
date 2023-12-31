// Exercise 1
// Make it compile
fn exercise1() {
    // Use as many approaches as you can to make it work
    let x = String::from("hello, world");
    let y = x;
    let z = y;
}

// Exercise 2
// Make it compile
// Don't modify code in exercise2 function!
fn exercise2() {
    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}
// Only modify the code below!
fn take_ownership(s: String) -> String {
    //println!("{}", s);
    s
}

// Exercise 3
// Make it compile
// Dont care about logic
fn exercise3() {
    let values: Vec<f64> = vec![
        2817.42, 2162.17, 3756.57, 2817.42, -2817.42, 946.9, 2817.42, 964.42, 795.43, 3756.57,
        139.34, 903.58, -3756.57, 939.14, 828.04, 1120.04, 604.03, 3354.74, 2748.06, 1470.8,
        4695.71, 71.11, 2391.48, 331.29, 1214.69, 863.52, 7810.01,
    ];

    let values_number: usize = values.len();

    let additions: Vec<usize> = vec![0];

    println!("{:?}", values_number);

    while additions.len() > 0 {
        let mut addition: f64 = 0.0;

        // Sumar valores en additions
        for element_index in additions.iter() {
            let addition_aux = values[*element_index];
            addition = addition_aux + addition;
        }
    }
}

// Exercise 4
// Make it compile
fn exercise4(value: u32) -> &'static str {
    let str_value = value.to_string(); // Convert u32 to String
    let str_ref: &str = &str_value; // Obtain a reference to the String
    "hi hi" // Return the reference to the String
}

// Exercise 5
// Make it compile
use std::collections::HashMap;
fn exercise5() {
    let mut my_map = HashMap::from([(1, "1.0".to_string()), (2, "2.0".to_string())]);

    let key = 3;
    let value = "3.0".to_string();
    let res = match my_map.get(&key) {
        Some(child) => child,
        None => {
            
            my_map.insert(key, value.clone());
            &value // HERE IT FAILS
        }
    };

    println!("{}", res);
}

// Exercise 6
// Make it compile

use std::io;

fn exercise6() {
    

    for line in io::stdin().lines() {
        let s = line.unwrap();
        let mut prev_key: &str = "";

        let data: Vec<&str> = s.split("\t").collect();
        if prev_key.len() == 0 {
            prev_key = data[0];
        }
    }
}

// Exercise 7
// Make it compile
fn exercise7() {
    
    {let mut v: Vec<&str> = Vec::new();
        let chars = [b'x', b'y', b'z'];
        let s: &str = std::str::from_utf8(&chars).unwrap();
        v.push(s.clone());
    }
    
}

// Exercise 8
// Make it compile
fn exercise8() {
    let mut accounting = vec!["Alice", "Ben"];
    let mut add_input= String::from("");
    let mut add_input1= String::from("");
    let mut add_vec: Vec<&str> = vec![" "];
    //add_input1 = add_input.clone();
    loop {
        let mut add_input= String::from("");
        

        io::stdin()
            .read_line(&mut add_input)
            .expect("Failed to read line");
        //add_input1 = add_input.clone();
         add_vec  = add_input1.trim()[..].split_whitespace().collect();

        if add_vec.len() < 1 {
            println!("Incorrect input, try again");
            continue;
        }

        let person = add_vec[0];
        accounting.push(person);
    }
}
