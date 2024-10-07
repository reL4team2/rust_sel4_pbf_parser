use std::fs;
use std::path::{Path, PathBuf};
use std::env;
use glob::glob;
use super::bf;
use proc_macro2::TokenStream;
use sel4_rustfmt_helper::Rustfmt;
fn pbf_parser(in_dir:String, out_dir:String) {
	let out_dir = OutDir::new(out_dir);

	let mut blocklist_for_bindgen = vec![];
    println!("start generate the rs,current path is {}!",env::current_dir().unwrap().display());
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
		println!("out_path is {}",out_path.display());
        fs::write(&out_path, format!("{fragment}")).unwrap();
        self.rustfmt.format(&out_path);
    }
}
