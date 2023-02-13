use std::{collections::HashMap, vec};

fn main() {
    //array
    let arr = [1, 2, 3];

    // vectors
    let mut vec = vec![1, 2, 3];
    let mut vec1 = Vec::new();
    vec1.push(1);

    let index_2 = &vec[1];

    match vec.get(2) {
        Some(index_2) => println!("This is index two!"),
        _ => println!("This is not index two!"),
    }

    for i in &mut vec {
        *i += 10;
    }

    for i in &vec {
        println!("{}", i);
    }

    // String

    let a = "hello";
    let b = a.to_string();
    let c = String::from("world");
    let d = String::from("hhh");
    let e = c + " 123 kf" + &d;
    println!("{}", e);
    println!("{}", a);

    // Hashmap

    let mu = String::from("MU");
    let mc = String::from("MC");

    let mut score = HashMap::new();
    score.insert(mu, 10);
    score.insert(mc, 9);

    score.entry(String::from("MU")).or_insert(30); //

    let mun = String::from("MU");

    for (k, v) in &score {
        println!("{} {}", k, v);
    }

    println!("{:?}", score.get(&mun));
}
