extern crate env_logger;
extern crate log;
//use backtrace::Backtrace;
use num::complex::Complex;

#[derive(Debug)]
struct AxisRange {
    min: f64,
    max: f64,
}

impl AxisRange {
    pub fn new(min: f64, max: f64) -> Result<AxisRange, String> {
        if min >= max {
            Err("Min value must be less than Max value".to_string())
        } else {
            Ok(AxisRange { min: min, max: max })
        }
    }
}

fn calculate_mandlebrot(
    max_iters: usize,
    x_axis: Result<AxisRange, String>,
    y_axis: Result<AxisRange, String>,
    width: usize,
    height: usize,
) -> Vec<Vec<usize>> {
    let mut rows: Vec<_> = Vec::with_capacity(width);
    match (x_axis, y_axis) {
        (Ok(x), Ok(y)) => {
            for img_y in 0..height {
                let mut row: Vec<usize> = Vec::with_capacity(height);
                for img_x in 0..width {
                    let x_percent = img_x as f64 / width as f64;
                    let y_percent = img_y as f64 / height as f64;
                    let cx = x.min + (x.max - x.min) * x_percent;
                    let cy = y.min + (y.max - y.min) * y_percent;
                    let escaped_at = mandlebrot_at_point(cx, cy, max_iters);
                    row.push(escaped_at);
                }

                rows.push(row);
            }
        }
        (a, b) => {
            //let bt = Backtrace::new();
            log::error!("Invalid Axis Ranges: {:?}, {:?}", a, b);
        }
    }
    rows
}

fn mandlebrot_at_point(cx: f64, cy: f64, max_iters: usize) -> usize {
    let mut z = Complex { re: 0.0, im: 0.0 };
    let c = Complex::new(cx, cy);
    for i in 0..=max_iters {
        if z.norm() > 2.0 {
            return i;
        }
        z = z * z + c;
    }
    max_iters
}

fn render_mandlebrot(escape_vals: Vec<Vec<usize>>) {
    for row in escape_vals {
        let mut line = String::with_capacity(row.len());
        for column in row {
            let val = match column {
                0..=2 => ' ',
                3..=5 => '`',
                6..=10 => '.',
                11..=30 => '*',
                31..=100 => '+',
                101..=200 => 'x',
                201..=400 => '$',
                401..=700 => '#',
                _ => '%',
            };
            line.push(val);
        }
        println!("{}", line);
    }
}

fn main() {
    env_logger::init();
    let mandlebrot = calculate_mandlebrot(
        1000,
        AxisRange::new(-2.0, 1.0),
        AxisRange::new(-1.0, 1.0),
        100,
        24,
    );
    render_mandlebrot(mandlebrot);

    let mandlebrot2 = calculate_mandlebrot(
        1000,
        AxisRange::new(2.0, 1.0),
        AxisRange::new(-1.0, 1.0),
        100,
        24,
    );
    render_mandlebrot(mandlebrot2);
}
