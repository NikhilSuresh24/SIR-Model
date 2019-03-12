use gnuplot::{AxesCommon, Caption, Figure, Graph, Color};
// use gnuplot::*;
use std::env::args;

const NUM_STEPS: i32 = 100;
const I_POP_PERCENT: f32 = 0.001;
const R_START_POP: f32 = 0.0;

fn main() {
    let args: Vec<String> = args().collect();
    let params = Params::new(&args);
    let mut s: Vec<f32> = vec![(params.total_population * (1.0 - I_POP_PERCENT)).round()];
    let mut i: Vec<f32> = vec![(params.total_population * I_POP_PERCENT).round()];
    let mut r: Vec<f32> = vec![R_START_POP];
    for _iter in 1..NUM_STEPS + 1 {
        step(&mut s, &mut i, &mut r, params.beta, params.gamma, params.dt);
    }
    create_graph(&s, &i, &r);
}

fn step(s: &mut Vec<f32>, i: &mut Vec<f32>, r: &mut Vec<f32>, dt: f32, beta: f32, gamma: f32) {
    let last_idx = s.len() - 1;
    s.push((s[last_idx] - beta * dt * s[last_idx] * i[last_idx]).round());
    i.push((i[last_idx] + (beta * s[last_idx] - gamma) * i[last_idx] * dt).round());
    r.push((r[last_idx] + gamma * i[last_idx] * dt).round());
}

struct Params {
    total_population: f32,
    beta: f32,
    gamma: f32,
    dt: f32,
}

impl Params {
    fn new(args: &[String]) -> Params {
        if args.len() < 5 {
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
            dt: args[4]
                .trim()
                .parse()
                .expect("Expected a float for dt, did not get one"),
        }
    }
}

fn create_graph(s: &[f32], i: &[f32], r: &[f32]) {
    let v: &Vec<i32> = &(1..NUM_STEPS).collect();
    let mut fg = Figure::new();
    fg.axes2d()
        .set_title("SIR", &[])
        .set_legend(Graph(1.0), Graph(1.0), &[], &[])
        .set_x_label("Time", &[])
        .set_y_label("Number of People", &[])
        .lines(v, s, &[Caption("S"), Color("yellow")])
        .lines(v, i, &[Caption("I"), Color("red")])
        .lines(v, r, &[Caption("R"), Color("green")]);
    fg.show();
}
