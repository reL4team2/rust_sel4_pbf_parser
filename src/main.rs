mod parser;
pub use parser::pbf_parser;
pub mod bf;
fn main() {
    let in_dir = String::from("./pbf");
    let out_dir = String::from("./pbf");
    pbf_parser(in_dir, out_dir);
}
