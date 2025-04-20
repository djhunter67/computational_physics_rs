use trapezoidal_integral::Integrate;

mod trapezoidal_integral;

fn main() {
    const N: f64 = 10.;
    const A: f64 = 0.0;
    const B: f64 = 2.0;

    let integrate = Integrate::new(N, A, B, Box::new(f));

    println!("Integral result rs: {}", integrate.trapezoidal());
}

fn f(x: f64) -> f64 {
    x.powf(4.) - 2. * x + 1.
}
