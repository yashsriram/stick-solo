use serde::{Deserialize, Serialize};

mod ceo;
mod fcn;
mod goal;

extern crate sticksolo;

use ceo::CEO;
use fcn::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Experiment {
    fcn: FCN,
    ceo: CEO,
}

fn run() -> Experiment {
    let mut fcn = FCN::new(vec![
        (3, Activation::Linear),
        (5, Activation::LeakyReLu(0.1)),
        (5, Activation::LeakyReLu(0.1)),
        (5, Activation::LeakyReLu(0.1)),
        (5, Activation::LeakyReLu(0.1)),
        (2, Activation::Linear),
    ]);

    let mut ceo = CEO::default();
    ceo.generations = 100;
    ceo.batch_size = 50;
    ceo.num_evalation_samples = 6;
    ceo.elite_frac = 0.25;
    ceo.initial_std = 3.0;
    ceo.noise_factor = 3.0;

    let _th_std = ceo.optimize(&mut fcn).unwrap();

    let exp = Experiment { fcn: fcn, ceo: ceo };

    exp
}

fn main() {
    use std::env;
    use std::fs::File;
    use std::io::BufReader;

    let args = env::args();
    let exp = if args.len() == 1 {
        // Run
        let exp = run();
        // Save
        use chrono::{Datelike, Timelike, Utc};
        let now = Utc::now();
        serde_json::to_writer(
            &File::create(format!(
                "{}-{}:{}.json",
                now.day(),
                now.month(),
                now.num_seconds_from_midnight()
            ))
            .unwrap(),
            &exp,
        )
        .unwrap();
        exp
    } else {
        if args.len() != 2 {
            panic!("Bad cmd line parameters.");
        }
        // Load from file
        let args = args.collect::<Vec<String>>();
        let file = File::open(&args[1]).unwrap();
        let reader = BufReader::new(file);
        let exp: Experiment = serde_json::from_reader(reader).unwrap();
        exp
    };
    println!("{:?}", exp);
    // TODO Visualize
}
