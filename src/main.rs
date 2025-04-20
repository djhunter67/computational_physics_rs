use std::{fs::File, io::Read};

use plotters::{
    chart::ChartBuilder,
    prelude::{BitMapBackend, IntoDrawingArea},
    series::LineSeries,
    style::{Color, IntoFont, BLACK, RED, WHITE},
};
use trapezoidal_integral::Integrate;

mod trapezoidal_integral;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // plotting

    const N: u32 = 10;
    const A: u32 = 0;
    const B: u32 = 2;

    let result = Integrate::new(Box::new(f)).trapezoidal(N, A, B);

    println!("Integral result rs: {result}");

    solve_5_1().unwrap();

    Ok(())
}

fn f(x: f64) -> f64 {
    x.powf(4.) - 2. * x + 1.
}

fn solve_5_1() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("cartesian-plot.png", (720, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    // Read in the file's contents of the file call velocities.txt
    let file = File::open("excercises/velocities.txt").expect("Unable to open file");
    let mut reader = std::io::BufReader::new(file);
    let mut contents = String::new();
    reader
        .read_to_string(&mut contents)
        .expect("Unable to read file");

    // Parse the contents into a hashmap of u8 and f64
    let mut velocities: Vec<f64> = Vec::new();
    let mut times: Vec<u8> = Vec::new();

    for line in contents.lines() {
        let mut parts = line.split_whitespace();
        if let (Some(x), Some(y)) = (parts.next(), parts.next()) {
            if let (Ok(x), Ok(y)) = (x.parse::<u8>(), y.parse::<f64>()) {
                velocities.push(y);
                times.push(x);
            }
        }
    }

    let result: Vec<f64> = Integrate::trap_idx(velocities.clone()).unwrap();

    println!("run: {:#?}", result[33]);

    let times: Vec<f32> = times.iter().map(|x| *x as f32).collect();
    let result: Vec<f32> = result.iter().map(|x| *x as f32).collect();

    let mut chart = ChartBuilder::on(&root)
        .caption("Velocities", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(
            times[0]..times[times.len() - 1],
            result[0]..result[result.len() - 1],
        )
        .unwrap();

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(
        (0..times.len()).map(|x| (times[x], result[x])),
        &RED,
    ))?;

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;

    // root.present()?;

    Ok(())
}
