use burn_onnx::ModelGen;

fn main() {
    // generate model
    ModelGen::new()
        .input("./src/model/mnist.onnx")
        .out_dir("model/")
        .run_from_script();
}
