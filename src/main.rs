use capture;

fn main() {
    let path = std::path::Path::new("_examples/javascript.js");
    let function_name = String::from("jsFunction");

    capture::from_function(&path, &function_name).unwrap();
}
