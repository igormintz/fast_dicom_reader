use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod consts;
mod os_utils;
use os_utils::get_dicom_paths_from_folder;
mod dicom_utils;
use dicom_utils::{read_dicom_file, extract_dicom_data};

#[derive(Parser)]
#[command(name = "Fast DICOM reader")]
#[command(version = "1.0")]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Read(ReadArgs),
}

#[derive(clap::Args)]
struct ReadArgs {
    #[arg(short, long, help = "Directory path to scan for files")]
    path: PathBuf,
}



fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    match args.command {
        Command::Read(read_args) => {
            let dicom_paths = get_dicom_paths_from_folder(read_args.path.to_str().unwrap())?;
            for path in dicom_paths {
                let dicom_obj = read_dicom_file(path.to_str().unwrap())?;
                let dicom_data = extract_dicom_data(dicom_obj, path);
                for (tag, value) in dicom_data.tags {
                    println!("Tag: {}, Value: {:?}", tag, value);
                }
                println!("pixel data: {:?}", dicom_data.pixel_data);
            }
        }
    }
    Ok(())
}

