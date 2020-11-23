use tch::{kind, IndexOp, Tensor};

fn main() {
    println!("==== From slice, print, requires_grad ================================");
    let t = Tensor::of_slice(&[3, 1, 4, 1, 5]);
    t.print();
    println!("requires grad {:?}", t.requires_grad());
    // t.save(Path::new("t.pt"));

    println!("==== From randn ================================");
    let t = Tensor::randn(&[5, 4], kind::FLOAT_CPU);
    t.print();
    println!("requires grad {:?}", t.requires_grad());

    println!("==== Reshape ================================");
    let view = t.view([10, 2]);
    t.print();
    view.print();

    println!("==== Broadcast ================================");
    let a = Tensor::ones(&[3, 4], kind::FLOAT_CPU);
    a.print();
    (&a * 2.5).print();
    (&a / 2).print();
    (&a + 5).print();
    (&a - 1).print();

    println!("==== Elementwise arithmetic ================================");
    let a = Tensor::ones(&[3, 4], kind::FLOAT_CPU);
    let b = Tensor::randn(&[3, 4], kind::FLOAT_CPU);
    a.print();
    b.print();
    (&a + &b).print();
    (&a - &b).print();
    (&a * &b).print();
    (&b / &a).print();

    println!("==== Elementwise ops ================================");
    let a = Tensor::randn(&[2, 3, 2], kind::FLOAT_CPU);
    a.max().print();

    println!("==== Linear Algebra ================================");
    let a = Tensor::randn(&[3, 3], kind::FLOAT_CPU);
    a.print();
    let b = a.inverse();
    b.print();
    a.matmul(&b).print();
    println!("{:?}", a.norm());
    a.transpose(0, 1).print();

    let a = Tensor::ones(&[2, 5], kind::FLOAT_CPU);
    let b = Tensor::ones(&[5, 1], kind::FLOAT_CPU);
    a.print();
    b.print();
    a.matmul(&b).print();
    let c = Tensor::ones(&[1, 4], kind::FLOAT_CPU);
    Tensor::chain_matmul(&[&a, &b, &c]).print();
    println!("==== From/To ================================");
    let a = Tensor::from(1.0);
    a.print();
    let a = Tensor::randn(&[2, 4], kind::FLOAT_CPU);
    a.print();
    let a: Vec<Vec<f32>> = Vec::from(a);
    println!("{:?}", a);

    println!("==== Indexing ================================");
    let a = Tensor::randn(&[2, 4], kind::FLOAT_CPU);
    a.print();
    println!("{:?}", a.i((0, 1)));
}
