#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixed<M, N>(self, orther: Point<M, N>) -> Point<T, N> {
        Point {
            x: self.x,
            y: orther.y,
        }
    }
}

fn main() {
    print!("Hello");
    let point1 = Point { x: 10, y: "truong" };
    let point2 = Point { x: 20, y: "phan" };
    let mixed = point1.mixed(point2);

    println!("Mixed: {:#?}", mixed);
}

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Error(E),
}
