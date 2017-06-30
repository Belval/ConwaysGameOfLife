extern crate rand;

use self::rand::Rng;

pub struct Array2d {
    vect: Vec<i32>,
    size_x: u32,
    size_y: u32,
}

impl Array2d {
    pub fn width(&self) -> u32 {
        self.size_x
    }
    pub fn height(&self) -> u32 {
        self.size_y
    }
    pub fn total_area(&self) -> u32 {
        self.size_x * self.size_y
    }
    pub fn element_at(&self, x:u32, y:u32) -> i32 {
        self.vect[(x + y * self.size_x) as usize]
    }
    pub fn set_element_at(&mut self, x:u32, y:u32, val:i32) {
        self.vect.insert((x + y * self.size_x) as usize, val);
    }
    pub fn get_surrounding_count(&mut self, x:u32, y:u32) -> i32 {
        let mut count: i32 = 0;
        if x == 0 || y == 0 {
            count
        } else {
            count += self.element_at(x + 1, y + 1);
            count += self.element_at(x + 1, y);
            count += self.element_at(x + 1, y - 1);
            count += self.element_at(x, y - 1);
            count += self.element_at(x - 1, y - 1);
            count += self.element_at(x - 1, y);
            count += self.element_at(x - 1, y + 1);
            count += self.element_at(x, y + 1);
            count
        }
    }
    pub fn set_random_elements(&mut self, count:u32, value:i32) {
        let mut rng = rand::thread_rng();
        for _ in 0..count {
            let x = rng.gen::<u32>() % self.width();
            let y = rng.gen::<u32>() % self.height();
            self.set_element_at(x, y, value);
        }
    }
    pub fn new(x:u32, y:u32) -> Array2d {
        Array2d {
            vect: vec![0; (x * y) as usize],
            size_x: x,
            size_y: y,
        }
    }
}