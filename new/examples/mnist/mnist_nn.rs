// This should rearch 97% accuracy.

extern crate tch;
use anyhow::Result;
use tch::{
    nn, nn::Init, nn::LinearConfig, nn::Module, nn::OptimizerConfig, nn::Sequential, Device,
};

const IMAGE_DIM: i64 = 784;
const HIDDEN_NODES: i64 = 128;
const LABELS: i64 = 10;

fn net(vs: &nn::Path) -> Sequential {
    nn::seq()
        .add(nn::linear(
            vs / "layer1",
            IMAGE_DIM,
            HIDDEN_NODES,
            Default::default(),
        ))
        .add_fn(|xs| xs.relu())
        .add(nn::linear(
            vs / "layer2",
            HIDDEN_NODES,
            LABELS,
            Default::default(),
        ))
}

pub fn run() -> Result<()> {
    // let m = tch::vision::mnist::load_dir("data")?;
    let vs1 = nn::VarStore::new(Device::Cpu);
    let net1 = net(&vs1.root());
    println!("{:?}", net1);
    println!("{:?}", vs1);
    println!("{:?}", vs1.len());
    println!("{:?}", vs1.trainable_variables());
    let mut vs2 = nn::VarStore::new(Device::Cpu);
    println!("{:?}", vs2);
    vs2.copy(&vs1)?;
    println!("{:?}", vs2);
    // let mut opt = nn::Adam::default().build(&vs, 2e-3)?;
    // for epoch in 1..200 {
    //     let loss = net
    //         .forward(&m.train_images)
    //         .cross_entropy_for_logits(&m.train_labels);
    //     opt.backward_step(&loss);
    //     let test_accuracy = net
    //         .forward(&m.test_images)
    //         .accuracy_for_logits(&m.test_labels);
    //     println!(
    //         "epoch: {:4} train loss: {:8.5} test acc: {:5.2}%",
    //         epoch,
    //         f64::from(&loss),
    //         100. * f64::from(&test_accuracy),
    //     );
    // }
    Ok(())
}
