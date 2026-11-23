use clap::Parser;
use chrono::Local;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "tsdir", version, about = "Create directories with timestamp suffix", long_about = None)]
struct Args {
    /// Base name for the directory (required)
    #[arg(value_name = "NAME")]
    name: String,

    /// Parent directory to create the timestamped directory in
    #[arg(short, long, value_name = "DIR", default_value = ".")]
    parent: PathBuf,

    /// Print the created directory path without creating it
    #[arg(short, long)]
    dry_run: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Generate timestamp in format: YYYY-MM-DD-HH-MM
    let timestamp = Local::now().format("%Y-%m-%d-%H-%M").to_string();
    let dir_name = format!("{}-{}", args.name, timestamp);
    let full_path = args.parent.join(&dir_name);

    if args.dry_run {
        println!("{}", full_path.display());
        return Ok(());
    }

    fs::create_dir_all(&full_path)?;
    println!("Created: {}", full_path.display());

    Ok(())
}