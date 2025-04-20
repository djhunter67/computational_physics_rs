pub struct Integrate {
    slices: f64,
    start: f64,
    end: f64,
    constant: f64,
    polynomial: Box<dyn Fn(f64) -> f64>,
}

impl Integrate {
    pub fn new(slices: f64, start: f64, end: f64, polynomial: Box<dyn Fn(f64) -> f64>) -> Self {
        Self {
            slices,
            start,
            end,
            constant: (end - start) / slices,
            polynomial,
        }
    }

    pub fn trapezoidal(&self) -> f64 {
        let mut s = 0.5 * (self.polynomial)(self.start) + 0.5 * (self.polynomial)(self.end);
        for k in 1..self.slices.floor() as u16 {
            s += (self.polynomial)(self.start + k as f64 * self.constant);
        }
        s * self.constant
    }
}

// Write the tests for the Integrate struct
#[cfg(test)]
mod tests {
    use crate::{f, trapezoidal_integral::Integrate};

    #[test]
    fn test_integrate() {
        let integrate = Integrate::new(10.0, 0.0, 2.0, Box::new(f));
        let result = integrate.trapezoidal();
        assert!((result - 4.50656).abs() < f64::EPSILON);
    }

    #[test]
    fn test_integrate_with_different_function() {
        let integrate = Integrate::new(100.0, 0.0, 2.0, Box::new(f));
        let result = integrate.trapezoidal();
        assert!((result - 4.40107).abs() < 1e-5);
    }
}
