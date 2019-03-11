use std::env::args;

const NUM_STEPS: i32 = 100;

fn main() {
    let args: Vec<String> = args().collect();
    let params = Params::new(&args);
    let mut S: Vec<f32> = Vec::new();
    let mut I: Vec<f32> = Vec::new();
    let mut R: Vec<f32> = Vec::new();
    for iter in 1..NUM_STEPS + 1 {
        step(&mut S, &mut I, &mut R, params.beta, params.gamma, params.dt);
    }
}

fn step(S: &mut Vec<f32>, I: &mut Vec<f32>, R: &mut Vec<f32>, dt: f32, beta: f32, gamma: f32) {
    let last_idx = S.len() - 1;
    S.push(S[last_idx] - beta * dt * S[last_idx] * I[last_idx]);
    I.push(I[last_idx] + (beta * S[last_idx] - gamma) * I[last_idx] * dt);
    R.push(R[last_idx] - gamma * I[last_idx] * dt);
}

struct Params {
    beta: f32,
    gamma: f32,
    dt: f32,
}

impl Params {
    fn new(args: &[String]) -> Params {
        if args.len() < 3 {
            panic!("Not enough arguments");
        }
        Params {beta: args[1].trim().parse().expect("Expected a float for beta, did not get one"),
        gamma: args[2].trim().parse().expect("Expected a float for gamma, did not get one"),
        dt: args[3].trim().parse().expect("Expected a float for dt, did not get one")
        }
    }
}
