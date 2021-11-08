// This file will contain all necessary functions pertaining to the calculator

#[derive(Debug)]
pub struct Calculator {
    num1: f32,
    num2: f32,
}

impl Calculator {
    pub fn new(num1: f32, num2: f32) -> Self {
        Self { num1, num2 }
    }

    pub fn add(&self) -> f32 {
        self.num1 + self.num2
    }

    pub fn multiply(&self) -> f32 {
        self.num1 * self.num2
    }

    pub fn subtract(&self) -> f32 {
        if self.num1 > self.num2 {
            self.num1 - self.num2
        } else {
            self.num2 - self.num1
        }
    }

    pub fn divide(&self) -> f32 {
        if self.num1 > self.num2 {
            self.num1 / self.num2
        } else {
            self.num2 / self.num1
        }
    }
}
