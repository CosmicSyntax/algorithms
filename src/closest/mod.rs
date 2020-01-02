/*
    This module takes an array of points on a 2D plain, and finds the
    closest pairs
*/

use std::fmt;

// fn main() {

//     let mut mp = Points::new();
//     mp.add(2,3);
//     mp.add(5,4);
//     mp.add(9,10);

//     println!("{}", mp);

// }

struct Point {
    x: u8,
    y: u8,
}

pub struct Points(Vec<Point>);

impl Points {
    pub fn new() -> Points {
        Points(Vec::new())
    }

    pub fn add(&mut self, x: u8, y: u8) {
        self.0.push(Point { x, y });
    }
}

impl fmt::Display for Points {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut text = String::new();

        for k in 0..self.0.len() {
            text = format!("{}{}. ({}, {})\n", text, k + 1, self.0[k].x, self.0[k].y);
        }

        write!(f, "{}", text)
    }
}

// Unit Test

mod tests {
    #[test]
    fn check_short() {
        unimplemented!();
    }
}