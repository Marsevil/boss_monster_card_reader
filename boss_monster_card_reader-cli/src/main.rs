use clap::Parser;
use std::path::PathBuf;

use boss_monster_card_reader_core::helpers::load_image;
use boss_monster_card_reader_core::read_batch;

mod diag;

#[derive(Debug, Clone, Parser)]
struct Args {
    /// Path to the card scans
    #[arg(short, long)]
    img_paths: Vec<PathBuf>,
    /// Path to the output file
    #[arg(short, long)]
    out_path: PathBuf,
    #[arg(long)]
    diag_folder_path: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    if cfg!(debug_assertions) {
        println!("Parsed Args :\n\t {:#?}", args);
    }

    // TODO: Check output type to be supported.

    for (idx, path) in args.img_paths.iter().enumerate() {
        let img = load_image(path)?;

        let diag_folder_path = args.diag_folder_path.as_ref().map(|path| {
            let folder_name = format!("scan_{}", idx);
            path.join(folder_name)
        });

        if cfg!(feature = "diag_reading") {
            if let Some(diag_folder_path) = diag_folder_path.as_ref() {
                const FILE_NAME: &str = "read.png";
                let file_path = diag_folder_path.join(FILE_NAME);
                img.save(file_path).unwrap();
            }
        }

        let diag = args
            .diag_folder_path
            .as_ref()
            .map(|diag_folder_path| diag::CliDiag {
                input_image: &img,
                output_path: diag_folder_path.to_path_buf(),
            });

        let infos = read_batch(&img, diag.as_ref());
    }

    Ok(())
}
