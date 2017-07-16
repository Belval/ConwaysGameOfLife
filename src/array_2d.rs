extern crate rand;

use self::rand::Rng;

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
        if x < self.size_x && x >= 0 && y < self.size_y && y >= 0 {
            self.vect[(x + y * self.size_x) as usize]
        } else {
            0
        }
    }
    pub fn set_element_at(&mut self, x:i32, y:i32, val:i32) {
        self.vect.insert((x + y * self.size_x) as usize, val);
    }
    pub fn get_surrounding_count(&mut self, x:i32, y:i32) -> i32 {
        let mut count: i32 = 0;
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
    pub fn set_random_elements(&mut self, count:i32, value:i32) {
        let mut rng = rand::thread_rng();
        for _ in 0..count {
            let x = rng.gen::<u32>() % self.width() as u32;
            let y = rng.gen::<u32>() % self.height() as u32;
            self.set_element_at(x as i32, y as i32, value);
        }
    }
    pub fn new(x:i32, y:i32) -> Array2d {
        Array2d {
            vect: vec![0; (x * y) as usize],
            size_x: x,
            size_y: y,
        }
    }
}