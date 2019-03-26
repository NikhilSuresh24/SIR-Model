use gnuplot::{AutoOption::Fix, AxesCommon, Caption, Color, Figure, Graph, TickOption::Mirror};
// use gnuplot::*;
use std::env::args;

const NUM_STEPS: i32 = 100;
const I_POP_PERCENT: f32 = 0.001;
const R_START_POP: f32 = 0.0;
const MIN_POP: f32 = 0.0;

fn main() {
    let args: Vec<String> = args().collect();
    let params = Params::new(&args);
    println!("beta: {}", params.beta);
    println!("gamma: {}", params.gamma);
    println!("alpha: {}", params.alpha);
    println!("dt: {}", params.dt);
    let mut s: Vec<f32> = vec![(params.total_population * (1.0 - I_POP_PERCENT)).round()];
    let mut i: Vec<f32> = vec![(params.total_population * I_POP_PERCENT).round()];
    let mut r: Vec<f32> = vec![R_START_POP];
    let mut d: Vec<f32> = vec![0.0];
    for _iter in 1..=NUM_STEPS {
        step(
            &mut s,
            &mut i,
            &mut r,
            &mut d,
            params.beta,
            params.gamma,
            params.dt,
            params.alpha,
        );
    }
    let last_idx = NUM_STEPS - 1;
    // println!(
    //     "S:{0}, I:{1}, R:{2}, D:{3}",
    //     s[last_idx], i[last_idx], r[last_idx], d[last_idx]
    // );
    // s = s.into_iter().map(|x| x.ln()).collect();
    // i = i.into_iter().map(|x| x.ln()).collect();
    // r = r.into_iter().map(|x| x.ln()).collect();
    // d = d.into_iter().map(|x| x.ln()).collect();

    create_graph(&s, &i, &r, &d);
    //print!("{},d);
}

fn step(
    s: &mut Vec<f32>,
    i: &mut Vec<f32>,
    r: &mut Vec<f32>,
    d: &mut Vec<f32>,
    dt: f32,
    beta: f32,
    gamma: f32,
    alpha: f32,
) {
    let last_idx = s.len() - 1;
    // println!(
    //     "B:{0}, dt:{1}, s:{2}, i:{3}, M:{4}",
    //     beta,
    //     dt,
    //     s[last_idx],
    //     i[last_idx],
    //     beta * dt * s[last_idx] * i[last_idx]
    // );
    // println!("addition: {}", beta * dt * s[last_idx] * i[last_idx]);
    s.push(MIN_POP.max(s[last_idx] - beta * dt * s[last_idx] * i[last_idx]));
    i.push(
        (MIN_POP.max(i[last_idx] + (beta * s[last_idx] - gamma) * i[last_idx] * dt)
            - i[last_idx] * alpha * dt),
    );
    r.push(MIN_POP.max(r[last_idx] + gamma * i[last_idx] * dt));
    d.push(MIN_POP.max(d[last_idx] + i[last_idx] * alpha * dt));
}

struct Params {
    total_population: f32,
    beta: f32,  //percent of s * infected who become infected
    gamma: f32, //percent of infected who recover
    alpha: f32, //percent of infected who become dead
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
            dt: args[5]
                .trim()
                .parse()
                .expect("Expected a float for dt, did not get one"),
        }
    }
}

fn create_graph(s: &[f32], i: &[f32], r: &[f32], d: &[f32]) {
    let v: &Vec<i32> = &(1..NUM_STEPS).collect();
    let mut fg = Figure::new();

    fg.axes2d()
        .set_title("SIR", &[])
        .set_legend(Graph(1.0), Graph(1.0), &[], &[])
        .set_x_label("Time", &[])
        .set_y_label("Number of People", &[])
        .lines(v, s, &[Caption("S"), Color("yellow")])
        .lines(v, i, &[Caption("I"), Color("red")])
        .lines(v, r, &[Caption("R"), Color("green")])
        .lines(v, d, &[Caption("D"), Color("black")]);
    fg.show();
}
