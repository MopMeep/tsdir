use clap::Parser;
use chrono::Local;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "tsdir", version, about = "Create directories with timestamp suffix", long_about = None)]
struct Args {
    /// Base name for the directory (optional - if omitted, uses only timestamp)
    #[arg(value_name = "NAME")]
    name: Option<String>,

    /// Parent directory to create the timestamped directory in
    #[arg(short, long, value_name = "DIR", default_value = ".")]
    parent: PathBuf,

    /// Print the created directory path without creating it
    #[arg(short, long)]
    dry_run: bool,

    /// Timestamp verbosity level: 1=date only, 2=date+time (default), 3=date+time+seconds
    #[arg(short = 'V', long, value_name = "LEVEL", default_value = "2", value_parser = clap::value_parser!(u8).range(1..=3))]
    verbosity: u8,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Generate timestamp based on verbosity level
    let timestamp_format = match args.verbosity {
        1 => "%Y-%m-%d",                    // 2026-11-23
        2 => "%Y-%m-%d-%H-%M",              // 2026-11-23-14-30 (default)
        3 => "%Y-%m-%d-%H-%M-%S",           // 2026-11-23-14-30-45
        _ => "%Y-%m-%d-%H-%M",
    };
    
    let timestamp = Local::now().format(timestamp_format).to_string();
    
    let dir_name = match args.name {
        Some(name) => format!("{}-{}", name, timestamp),
        None => timestamp,
    };
    
    let full_path = args.parent.join(&dir_name);

    if args.dry_run {
        println!("{}", full_path.display());
        return Ok(());
    }

    fs::create_dir_all(&full_path)?;
    println!("Created: {}", full_path.display());

    Ok(())
}