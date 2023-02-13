fn main() {
    println!("Hello");
    let a = "truong";
    let b = String::from("truong");
    let result = choose_str(1, 2, a, "phan");
    let result2 = choose_str_2(1, 2, &b, String::from("phan"));

    println!("result1 = {}", result);
    println!("result2 = {}", result2);
    println!(" {}", a);
    println!("{}", b);
}

fn choose_str<'a>(p1: i32, p2: i32, r1: &'a str, r2: &'a str) -> &'a str {
    if p1 > p2 {
        r1
    } else {
        r2
    }
}

fn choose_str_2(p1: i32, p2: i32, r1: &String, r2: String) -> String {
    if p1 > p2 {
        r1.clone()
    } else {
        r2
    }
}
