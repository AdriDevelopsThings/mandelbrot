pub struct ComplexNumber {
    re: f64,
    im: f64,
}

impl ComplexNumber {
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    pub fn null() -> Self {
        Self::new(0f64, 0f64)
    }

    // self ^ 2
    pub fn square(self) -> Self {
        Self {
            re: (self.re * self.re) - (self.im * self.im),
            im: 2f64 * self.re * self.im,
        }
    }

    // self + other
    pub fn add(self, other: &Self) -> Self {
        Self {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }

    // (|self|)
    pub fn abs_square(&self) -> f64 {
        (self.re * self.re) + (self.im * self.im)
    }
}
