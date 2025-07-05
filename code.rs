use std::error::Error;
use std::fs::File;
use csv::Writer;
use plotters::prelude::*;

// dy/dt = cos(t) - y
fn f(t: f64, y: f64) -> f64 {
    t.cos() - y
}

// Exact solution
fn exact_solution(t: f64) -> f64 {
    0.5 * (t.cos() + t.sin()) + 0.5 * (-t).exp()
}

fn main() -> Result<(), Box<dyn Error>> {
    let a = 0.0;
    let b = 5.0;
    let n = 1000;
    let h = (b - a) / n as f64;

    let mut t_vals = Vec::new();
    let mut y_euler = Vec::new();
    let mut y_exact = Vec::new();
    let mut errors = Vec::new();

    let mut t = a;
    let mut y = 1.0;

    for _ in 0..=n {
        let ye = exact_solution(t);
        let err = (ye - y).abs();

        t_vals.push(t);
        y_euler.push(y);
        y_exact.push(ye);
        errors.push(err);

        y += h * f(t, y);
        t += h;
    }

    // Write to CSV
    let mut wtr = Writer::from_path("solution.csv")?;
    wtr.write_record(&["t", "euler_y", "exact_y", "error"])?;
    for i in 0..=n {
        wtr.write_record(&[
            t_vals[i].to_string(),
            y_euler[i].to_string(),
            y_exact[i].to_string(),
            errors[i].to_string(),
        ])?;
    }
    wtr.flush()?;
    println!("Data saved to solution.csv");

    // Plotting
    let root = BitMapBackend::new("plot.png", (900, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let y_min = y_euler
        .iter()
        .chain(y_exact.iter())
        .cloned()
        .fold(f64::INFINITY, f64::min);
    let y_max = y_euler
        .iter()
        .chain(y_exact.iter())
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max);

    let mut chart = ChartBuilder::on(&root)
        .caption("Euler vs Exact Solution", ("sans-serif", 25))
        .margin(20)
        .x_label_area_size(30)
        .y_label_area_size(50)
        .build_cartesian_2d(a..b, y_min..y_max)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            t_vals.iter().zip(y_euler.iter()).map(|(&t, &y)| (t, y)),
            &BLUE,
        ))?
        .label("Euler")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart
        .draw_series(LineSeries::new(
            t_vals.iter().zip(y_exact.iter()).map(|(&t, &y)| (t, y)),
            &GREEN,
        ))?
        .label("Exact")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    println!("✅ Plot saved as plot.png");

    Ok(())
}
