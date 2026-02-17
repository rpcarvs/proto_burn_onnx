mod model;

use std::env::args;

use burn::{
    backend::ndarray::NdArray,
    data::dataset::{Dataset, vision::MnistDataset},
    tensor::Tensor,
};

use crate::model::mnist::Model;

const IMAGE_INX: usize = 42; // <- Change this to test a different image'

fn main() {
    // get image argument at runtime
    let image_index = if let Some(image_index) = args().nth(1) {
        println!("Image Index: {image_index}");
        image_index
            .parse::<usize>()
            .expect("Failed to parse image index")
    } else {
        println!("No image index provided! Using default.");
        IMAGE_INX
    };
    assert!(image_index < 10000, "Image index must be less than 10000");

    //start inference
    type Backend = NdArray<f32>;
    let device = <Backend as burn::tensor::backend::Backend>::Device::default();
    let model: Model<Backend> = Model::default();

    // load dataset and retrieve an item
    let dataset = MnistDataset::test();
    let item = dataset.get(image_index).unwrap();

    // create tensor from image
    let image_data = item.image.iter().copied().flatten().collect::<Vec<f32>>();
    let mut input =
        Tensor::<Backend, 1>::from_floats(image_data.as_slice(), &device).reshape([1, 1, 28, 28]);

    // Normalize the input
    input = ((input / 255) - 0.1307) / 0.3081;

    // now run the model
    let output = model.forward(input);
    let arg_max = output.argmax(1).into_scalar() as u8;

    // Check if the index matches the label
    assert!(arg_max == item.label);

    println!("Success!");
    println!("Predicted: {arg_max}");
    println!("Actual: {}", item.label);
    println!("See the image online, click the link below:");
    println!("https://huggingface.co/datasets/ylecun/mnist/viewer/mnist/test?row={image_index}");
}
