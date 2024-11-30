use std::collections::HashMap;

struct CompressionModel {
    encoders: HashMap<>
}

fn compression_length(data: &[u8]) -> Vec<u8> {
    data
        .iter()
        .cloned()
        .encode(&mut BZip2Encoder::new(9), Action::Finish)
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

fn ndc(x: &[u8], y: &[u8]) -> f64 {

}

fn main() {
    println!("Hello, world!");
}
