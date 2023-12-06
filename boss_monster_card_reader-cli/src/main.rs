use clap::Parser;
use std::path::PathBuf;

use boss_monster_card_reader_core::helpers::load_image;
use boss_monster_card_reader_core::read_batch;

#[derive(Debug, Clone, Parser)]
struct Args {
    /// Path to the card scans
    #[arg(short, long)]
    img_paths: Vec<PathBuf>,
    /// Path to the output file
    #[arg(short, long)]
    out_path: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    if cfg!(debug_assertions) {
        println!("Parsed Args :\n\t {:?}", args);
    }

    // TODO: Check output type to be supported.

    for path in &args.img_paths {
        let img = load_image(path)?;
        let infos = read_batch(&img)?;
    }

    Ok(())
}
