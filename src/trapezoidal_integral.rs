pub struct Integrate {
    polynomial: Box<dyn Fn(f64) -> f64>,
}

impl Integrate {
    pub fn new(polynomial: Box<dyn Fn(f64) -> f64>) -> Self {
        Self { polynomial }
    }

    pub fn trapezoidal(&self, slices: u32, start: u32, end: u32) -> f64 {
        let slices_f64 = f64::from(slices);
        let constant = f64::from(end - start) / slices_f64;
        let mut s =
            0.5 * (self.polynomial)(f64::from(start)) + 0.5 * (self.polynomial)(f64::from(end));
        for k in 1..slices {
            s += (self.polynomial)(f64::from(start) + k as f64 * constant);
        }
        s * constant
    }

    pub fn trap_idx(strt_end: Vec<f64>) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
        let mut result = Vec::with_capacity(strt_end.clone().len());

        result.push(0.); // the first idx

        for k in 1..strt_end.len() {
            result.push(result[k - 1] + (strt_end[k] + strt_end[k - 1]) / 2.);
        }

        Ok(result)
    }
}

// Write the tests for the Integrate struct
#[cfg(test)]
mod tests {
    use crate::{f, trapezoidal_integral::Integrate};

    #[test]
    fn test_integrate() {
        let integrate = Integrate::new(Box::new(f));
        let result = integrate.trapezoidal(10, 0, 2);
        assert!((result - 4.50656).abs() < f64::EPSILON);
    }

    #[test]
    fn test_integrate_with_different_function() {
        let integrate = Integrate::new(Box::new(f));
        let result = integrate.trapezoidal(100, 0, 2);
        assert!((result - 4.40107).abs() < 1e-5);
    }
}
