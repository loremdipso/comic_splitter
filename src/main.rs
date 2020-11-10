mod splitter;
use crate::splitter::Splitter;

use rayon::prelude::*;
use std::path::Path;
use std::path::PathBuf;
use std::time::SystemTime;
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
	#[structopt(parse(from_os_str))]
	input_paths: Vec<PathBuf>,

	#[structopt(parse(from_os_str), long, short)]
	output: PathBuf,

	#[structopt(
		short,
		long,
		help = "Normally we assume white lines divide the image. Use this option to instead use black lines"
	)]
	black_lines: bool,

	#[structopt(long, short, help = "Delete the original file")]
	delete: bool,
}

fn main() {
	let args = Cli::from_args();

	for path in args.input_paths.iter() {
		if !Path::new(&path).exists() {
			println!("path doesn't exist: {:?}", path);
			return;
		}
	}

	args.input_paths.par_iter().for_each(|image| {
		let then = SystemTime::now();
		Splitter {
			black_lines: args.black_lines,
			remove_original: args.delete,
		}
		.split_image(args.output.clone(), image.clone());

		println!(
			"Finished in {}ms",
			SystemTime::now().duration_since(then).unwrap().as_millis()
		);
	});
}
