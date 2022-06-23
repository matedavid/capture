use capture;

fn main() {
    let path = std::path::Path::new("_examples/rust.rs");
    let function_name = String::from("indented_function");

    let mut cap = capture::Capture::new(&path).unwrap();
    cap.from_function(&function_name).unwrap();
    cap.print();
}
