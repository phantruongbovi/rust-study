fn  main() {
    fn _x() {
        let mut _a = "truong";
       // _a.push_str("!"); error because _a not string, it is array character
    }

    fn _y() {
        let mut _a = String::from("truong");
        _a.push_str("!");
    }

    // One variable one owner

    let _s1 = String::from("Hello");
    let _s2 = _s1.clone();
    // println!("{}", _s1); //value borrowed here after move
    println!("{}", _s1);


    let s = String::from("Hello");
    let s_back = takes_ownership_and_back(s);
    println!("{}", s_back);


    // advanced
    // referal for
    let mut s1 = String::from("hello");
    let len_str = calculate_len(&mut s1);
    println!("{} = {}", s1, len_str);
}

fn calculate_len(_string: &mut String) -> usize {
    _string.push_str("world");
    let len = _string.len();
    len
}

// mượn không trả
fn takes_ownership(_string: String) {
    println!("{}", _string);
}

// mượn trả value to new ownership
fn takes_ownership_and_back(_string: String) -> String {
    _string
}