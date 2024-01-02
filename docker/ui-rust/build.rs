use style4rs::Builder;

use std::path::Path;

pub fn main() {
    let out_file = Path::new("target/css-generated/main.css");
    Builder::new().using_out_file(out_file).build().ok();
}