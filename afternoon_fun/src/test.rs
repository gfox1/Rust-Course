use std::fmt;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let mut v = vec![];
    for x in 0..10 {
        v.push(Point {
            x,
            y: 5
        })
    }
    let p = Point { x: 4,y: 5};

    let u: i32;
    u = 5;

    println!("{}", u);
}