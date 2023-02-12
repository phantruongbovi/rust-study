#[derive(Debug)]
struct Square {
    dai: u32,
    rong: u32,
}

impl Square {
    fn dien_tich(&self) -> u32 {
        return self.dai * self.rong
    }
}

fn main() {
    let square1 = Square {
        dai: 20,
        rong: 10,
    };
    println!("{}", square1.dien_tich());
}


// struct Member {
//     username: String,
//     email: String,
//     age: u64,
//     active: bool,
//     sex: bool,
// }
// fn main() {
//     let mut member1 = Member {
//         email: String::from("phantruong@gmail.com"),
//         username: String::from("Truong"),
//         age: 23,
//         active: true,
//         sex: true,
//     };

//     let name = member1.username;
//     println!("{}", name);
//     member1.username = String::from("Phan");
//     println!("{}", member1.username);

//     let member2 = create_new_account(String::from("truong"), String::from("pttruong@gmail.com"), 23);
//     println!("member2 = {:?}", member2);
//     println!("member2 = {:#?}", member2);

//     let member3 = Member {
//         username: String::from("Phan"),
//         ..member2
//     };
//     println!("member3 = {:#?}", member3);

// }

// fn create_new_account(username: String, email: String, age: u64) -> Member {
//     Member {
//         email,
//         username,
//         age,
//         active: true,
//         sex: true,
//     }
// }