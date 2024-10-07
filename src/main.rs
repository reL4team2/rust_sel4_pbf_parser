use std::fs;
use std::path::{Path, PathBuf};
use std::env;
use glob::glob;
mod bf;
use proc_macro2::TokenStream;
use sel4_rustfmt_helper::Rustfmt;
fn main() {
	let out_dir = OutDir::new();

	let mut blocklist_for_bindgen = vec![];
    println!("start generate the rs,current path is {}!",env::current_dir().unwrap().display());
	for f in glob(&format!("./bf/*.pbf"))
		.unwrap()
		.map(Result::unwrap)
	{
		let (native_fragment, _wrappers_fragment) =
			bf::generate_rust(&mut blocklist_for_bindgen, &f);
		out_dir.write_file(native_fragment, f.with_extension("rs").file_name().unwrap());
		// out_dir.write_file(
		// 	wrappers_fragment,
		// 	f.with_extension("wrappers.rs").file_name().unwrap(),
		// );
	}
}
struct OutDir {
    path: PathBuf,
    rustfmt: Rustfmt,
}

impl OutDir {
    fn new() -> Self {
        Self {
            path: Path::new("./bf").to_owned(),
            rustfmt: Rustfmt::detect(),
        }
    }

    fn write_file(&self, fragment: TokenStream, filename: impl AsRef<Path>) {
        let out_path = self.path.join(filename);
		println!("out_path is {}",out_path.display());
        fs::write(&out_path, format!("{fragment}")).unwrap();
        self.rustfmt.format(&out_path);
    }
}
