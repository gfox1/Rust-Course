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
 
    // Mutable variable
        // let mut temp: i32 = 22;
        // temp = 18;

    // Array of fixed size
        // let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    
    // Vector of unknown size
        // let mut vec: Vec<i32> = Vec::new();
        // vec.push(23);

    // Create vector using a macro
        // let mut vec: Vec<i32> = vec![1, 2, 3, 4, 5];
        // vec.push(6);

   
