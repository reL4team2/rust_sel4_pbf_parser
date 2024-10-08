use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::env;
use glob::glob;
use super::bf;
use proc_macro2::TokenStream;
use sel4_rustfmt_helper::Rustfmt;
pub fn pbf_parser(in_dir:String, out_dir:String) {
	let out_dir = OutDir::new(out_dir);

	let mut blocklist_for_bindgen = vec![];
	for f in glob(&format!("{}/*.pbf",in_dir))
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
    fn new(out_dir:String) -> Self {
        Self {
            path: Path::new(out_dir.as_str()).to_owned(),
            rustfmt: Rustfmt::detect(),
        }
    }

    fn write_file(&self, fragment: TokenStream, filename: impl AsRef<Path>) {
        let out_path = self.path.join(filename);
		let mut file = OpenOptions::new()  
			.append(true)
			.create(true)
			.open(out_path.clone())
			.expect("cannot open file");
		let context = format!("
			use core::fmt;
			use sel4_bitfield_types::Bitfield;
		");
        file.write_all(context.as_bytes()).unwrap();
		let context = format!("{fragment}");
        file.write_all(context.as_bytes()).unwrap();
        self.rustfmt.format(&out_path.clone());
    }
}
