extern crate rand;

pub struct Array2d {
    vect: Vec<i32>,
    size_x: i32,
    size_y: i32,
}

impl Array2d {
    pub fn width(&self) -> i32 {
        self.size_x
    }
    pub fn height(&self) -> i32 {
        self.size_y
    }
    pub fn total_area(&self) -> i32 {
        self.size_x * self.size_y
    }
    pub fn element_at(&self, x:i32, y:i32) -> i32 {
        self.vect[(x + y * self.size_x) as usize]
    }
    pub fn set_element_at(&mut self, x:i32, y:i32, val:i32) {
        self.vect.insert((x + y * self.size_x) as usize, val);
    }
    pub fn new(x:i32, y:i32) -> Array2d {
        Array2d {
            vect: vec![0; (x * y) as usize],
            size_x: x,
            size_y: y,
        }
    }
}