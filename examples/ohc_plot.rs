extern crate stick_solo;
use bevy::prelude::*;
use image::{Rgb, RgbImage};
use std::ops::RangeInclusive;
use std::{env, fs::File, io::BufReader};
use stick_solo::act::one_holding_switchable_nr_couple::OneHoldingSwitchableNRCouple;
use stick_solo::plan::cross_entropy_optimizing::experiment::Experiment;
use stick_solo::plan::cross_entropy_optimizing::utils::{decode, encode};

fn main() {
    let args = env::args();
    if args.len() != 2 {
        panic!("Bad cmd line parameters.");
    }
    // Load from file
    let args = args.collect::<Vec<String>>();
    let experiment: Experiment =
        serde_json::from_reader(BufReader::new(File::open(&args[1]).unwrap())).unwrap();
    sample_holding_goals(experiment, (-40, 15), (-50, 50), 60.0);
}

fn pixel(x: f32, y: f32, xi: f32, yi: f32) -> [u8; 3] {
    if x.abs() < 1e-2 && y.abs() < 1e-2 {
        return [255, 255, 255];
    }
    if xi.abs() < 1e-2 && yi.abs() < 1e-2 {
        return [255, 255, 255];
    }
    let x = (x + y).floor() as i32;
    let abs_rem = x.abs() % 3;
    if abs_rem == 0 {
        [255, 0, 0]
    } else if abs_rem == 1 {
        [0, 255, 0]
    } else {
        [0, 0, 255]
    }
}

fn sample_holding_goals(
    experiment: Experiment,
    xrange: (i32, i32),
    yrange: (i32, i32),
    scale: f32,
) {
    assert!(xrange.0 < xrange.1, "Bad range argument.");
    assert!(yrange.0 < yrange.1, "Bad range argument.");
    assert!(scale > 0.0, "Bad scale argument.");
    let world = experiment.world;
    let agent = OneHoldingSwitchableNRCouple::new(
        &world.holding_side,
        Vec2::new(0.0, 0.0),
        &world.holding_ls,
        &world.sample_holding_qs(),
        &world.holding_q_clamps(),
        &world.non_holding_ls,
        &world.sample_non_holding_qs(),
        &world.non_holding_q_clamps(),
        0.01,
    );
    let fcn = experiment.fcn;
    println!(
        "x = {:?};",
        RangeInclusive::new(xrange.0, xrange.1)
            .map(|e| e as f32 / scale)
            .collect::<Vec<f32>>()
    );
    println!(
        "y = {:?};",
        RangeInclusive::new(yrange.0, yrange.1)
            .clone()
            .map(|e| e as f32 / scale)
            .collect::<Vec<f32>>()
    );
    let mut a: Vec<f32> = vec![];
    let mut b: Vec<f32> = vec![];
    let mut d: Vec<f32> = vec![];
    let mut o: Vec<(Vec2, Vec2)> = vec![];
    let mut original = RgbImage::new(
        (xrange.1 - xrange.0 + 1) as u32,
        (yrange.1 - yrange.0 + 1) as u32,
    );
    let mut mapped = RgbImage::new(
        (xrange.1 - xrange.0 + 1) as u32,
        (yrange.1 - yrange.0 + 1) as u32,
    );
    for x in RangeInclusive::new(xrange.0, xrange.1) {
        for y in RangeInclusive::new(yrange.0, yrange.1) {
            let (x, y) = (x as f32 / scale, y as f32 / scale);
            let non_holding_goal = Vec2::new(x, y);
            // Network pipeline
            let (input, fp_scale) = encode(&agent, &non_holding_goal);
            let forward_pass = fcn.at(&input);
            // Storing in vecs
            let input = Vec2::new(input[4], input[5]);
            let output = Vec2::new(forward_pass[0], forward_pass[1]);
            o.push((input, output));
            d.push(output.length());
            let output = output.normalize();
            a.push(output[0]);
            b.push(output[1]);
            // Image
            original.put_pixel(
                (x * scale - xrange.0 as f32) as u32,
                (yrange.1 as f32 - y * scale) as u32,
                Rgb(pixel(x, y, x, y)),
            );
            let output = decode(&forward_pass, fp_scale, Vec2::new(0.0, 0.0));
            mapped.put_pixel(
                (x * scale - xrange.0 as f32) as u32,
                (yrange.1 as f32 - y * scale) as u32,
                Rgb(pixel(output[0], output[1], x, y)),
            );
        }
    }
    println!("a = {:?};", a);
    println!("b = {:?};", b);
    println!("d = {:?};", d);
    original.save("original.png").unwrap();
    mapped.save("mapped.png").unwrap();
}
