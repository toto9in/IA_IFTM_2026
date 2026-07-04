#[derive(Clone, Copy)]
pub struct TermoTrapezoidal {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
}

impl TermoTrapezoidal {
    pub const fn new(a: f64, b: f64, c: f64, d: f64) -> Self {
        Self { a, b, c, d }
    }

    pub fn grau(&self, x: f64) -> f64 {
        if x < self.a || x > self.d {
            return 0.0;
        }
        if x >= self.b && x <= self.c {
            return 1.0;
        }
        if x < self.b {
            (x - self.a) / (self.b - self.a)
        } else {
            (self.d - x) / (self.d - self.c)
        }
    }
}
