struct Access {
    position: Position,
    authen: Authen,
}

enum Position {
    IT,
    CEO,
    OTO,
}

enum Authen {
    ACCESS,
    DENY,
}

fn validate_position(access: &Access) -> Result<(), String> {
    match access.authen {
        Authen::DENY => return Err("Access deny".to_owned()),
        _ => (),
    }

    match access.position {
        Position::IT => Ok(()),
        Position::CEO => Ok(()),
        _ => Err("invalid position".to_owned()),
    }
}

fn main() {
    println!("Hello");
    let access1 = Access {
        position: Position::IT,
        authen: Authen::DENY,
    };
    let result1 = validate_position(&access1);
    println!("Result {:?}", result1);
}
