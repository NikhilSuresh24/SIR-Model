use gnuplot::{AxesCommon, Caption, Color, Figure, Graph};
extern crate graphics;
extern crate image;
use std::env::args;
use std::ops::Sub;
use std::{f32, thread, time};

const I_START_POP: f32 = 2.0;
const R_START_POP: f32 = 0.0;
const MIN_POP: f32 = 0.0;
const WAIT_TIME: time::Duration = time::Duration::from_millis(125);

fn main() {
    let args: Vec<String> = args().collect();
    let params = Params::new(&args);
    println!("beta: {}", params.beta);
    println!("gamma: {}", params.gamma);
    println!("alpha: {}", params.alpha);
    println!("dt: {}", params.dt);
    println!("delta: {}", params.delta);

    let mut s: Vec<f32> = vec![params.total_population - I_START_POP];
    let mut i: Vec<f32> = vec![I_START_POP];
    let mut r: Vec<f32> = vec![R_START_POP];
    let mut d: Vec<f32> = vec![0.0];
    let mut fg = init_graph();
    let br = params.beta * s[0] / params.gamma; // basic reproductive ratio
    let hi = 1.0 - 1.0 / br; //Herd Immunity Threshold
    println!("Basic Reproductive Ratio: {}", br);
    println!("Herd Immunity Threshold: {}", hi);
    while i[i.len() - 1] > 0.5 {
        let start = time::Instant::now();
        step(
            &mut s,
            &mut i,
            &mut r,
            &mut d,
            params.beta,
            params.gamma,
            params.alpha,
            params.delta,
            params.dt,
        );
        if start.elapsed() < WAIT_TIME {
            thread::sleep(WAIT_TIME.sub(start.elapsed()));
        }
        update_graph(&mut fg, &s, &i, &r, &d, params.dt);
    }
}

#[allow(clippy::too_many_arguments)]
fn step(
    s: &mut Vec<f32>,
    i: &mut Vec<f32>,
    r: &mut Vec<f32>,
    d: &mut Vec<f32>,
    beta: f32,
    gamma: f32,
    alpha: f32,
    delta: f32,
    dt: f32,
) {
    let last_idx = s.len() - 1;
    s.push(
        MIN_POP.max(s[last_idx] - beta * dt * s[last_idx] * i[last_idx] - s[last_idx] * delta * dt),
    );
    i.push(MIN_POP.max(
        i[last_idx] + (beta * s[last_idx] - gamma) * i[last_idx] * dt - delta * s[last_idx] * dt,
    ));
    r.push(MIN_POP.max(r[last_idx] + gamma * i[last_idx] * dt + s[last_idx] * delta * dt));
    d.push(MIN_POP.max(d[last_idx] + i[last_idx] * alpha * dt));
}

struct Params {
    total_population: f32,
    beta: f32,  //percent of s * infected who become infected
    gamma: f32, //percent of infected who recover
    alpha: f32, //percent of infected who become dead
    delta: f32, //percent of vaccinated
    dt: f32,
}

impl Params {
    fn new(args: &[String]) -> Params {
        if args.len() < 6 {
            panic!("Not enough arguments");
        }
        Params {
            total_population: args[1]
                .trim()
                .parse()
                .expect("Expected a float for beta, did not get one"),
            beta: args[2]
                .trim()
                .parse()
                .expect("Expected a float for beta, did not get one"),
            gamma: args[3]
                .trim()
                .parse()
                .expect("Expected a float for gamma, did not get one"),
            alpha: args[4]
                .trim()
                .parse()
                .expect("Expected a float for alpha, did not get one"),
            delta: args[5]
                .trim()
                .parse()
                .expect("Expected a float for alpha, did not get one"),
            dt: args[6]
                .trim()
                .parse()
                .expect("Expected a float for dt, did not get one"),
        }
    }
}

fn init_graph() -> Figure {
    let mut fg = Figure::new();
    fg.axes2d()
        .set_title("SIR", &[])
        .set_legend(Graph(1.0), Graph(1.0), &[], &[])
        .set_x_label("Time", &[])
        .set_y_label("Number of People", &[]);
    fg.set_terminal(&"pngcairo", &"test2.png");
    fg.show();
    fg
}

fn update_graph(fg: &mut Figure, s: &[f32], i: &[f32], r: &[f32], d: &[f32], dt: f32) {
    let x_axis: &Vec<f32> = &(1..=s.len() as i32).map(|x| x as f32 * dt).collect();
    fg.clear_axes();
    fg.axes2d()
        .lines(x_axis, s, &[Caption("S"), Color("blue")])
        .lines(x_axis, i, &[Caption("I"), Color("red")])
        .lines(x_axis, r, &[Caption("R"), Color("green")])
        .lines(x_axis, d, &[Caption("D"), Color("black")]);
    fg.show();
}
