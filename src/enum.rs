#[derive(Debug)]
struct User {
    username: String,
    usertype: UserType,
    levelrank: u32,
}

#[derive(Debug)]
enum UserType {
    Small,
    Big,
    Shark,
}

fn main() {
    let user1 = User {
        username: String::from("Truong"),
        usertype: UserType::Big,
        levelrank: level_rank(UserType::Big),
    };
    println!("user1 = {:#?}", user1);

    // Option
    let a = Some(1);
    println!("a is valid = {}", check_valid_value(a))
}

fn check_valid_value(v: Option<i32>) -> bool {
    match v {
        Some(v) => true,
        _ => false,
    }
}

fn level_rank(usertype: UserType) -> u32 {
    match usertype {
        UserType::Small => 1,
        UserType::Big => 2,
        UserType::Shark => 3,
    }
}
