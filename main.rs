use clap::{Parser, Subcommand};
use std::path::PathBuf;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

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
    #[arg(short, long, help = "Number of threads to use for parallel processing (defaults to CPU cores - 1)")]
    threads: Option<usize>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    match args.command {
        Command::Read(read_args) => {
            // Determine number of threads
            let num_cores = num_cpus::get();
            let num_threads = read_args.threads.unwrap_or_else(|| {
                if num_cores > 1 {
                    num_cores - 1
                } else {
                    1
                }
            });
            
            println!("CPU cores detected: {}", num_cores);
            println!("Using {} threads for parallel processing", num_threads);
            
            // Set the number of threads for rayon
            rayon::ThreadPoolBuilder::new()
                .num_threads(num_threads)
                .build_global()
                .unwrap();

            let dicom_paths = get_dicom_paths_from_folder(read_args.path.to_str().unwrap())?;
            println!("Found {} DICOM files to process", dicom_paths.len());
            
            // Create a shared counter for progress reporting
            let processed_count = Arc::new(Mutex::new(0));
            let total_files = dicom_paths.len();
            
            // Process files in parallel
            let results: Vec<Result<(), Box<dyn std::error::Error + Send + Sync>>> = dicom_paths
                .into_par_iter()
                .map(|path| {
                    let result = process_single_dicom(&path);
                    
                    // Update progress counter
                    {
                        let mut count = processed_count.lock().unwrap();
                        *count += 1;
                        println!("Processed {}/{} files", *count, total_files);
                    }
                    
                    result
                })
                .collect();
            
            // Report any errors that occurred during processing
            let errors: Vec<_> = results.into_iter().filter_map(|r| r.err()).collect();
            if !errors.is_empty() {
                println!("\nEncountered {} errors during processing:", errors.len());
                for error in errors {
                    eprintln!("Error: {}", error);
                }
            } else {
                println!("\nSuccessfully processed all {} files!", total_files);
            }
        }
    }
    Ok(())
}

fn process_single_dicom(path: &PathBuf) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let path_str = path.to_str().unwrap();
    let dicom_obj = match read_dicom_file(path_str) {
        Ok(obj) => obj,
        Err(e) => {
            eprintln!("Failed to read DICOM file {}: {}", path_str, e);
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to read DICOM file: {}", e)
            )));
        }
    };
    
    let dicom_data = extract_dicom_data(dicom_obj, path.clone());
    
    // Print file information
    // println!("File: {}", path_str);
    // for (tag, value) in dicom_data.tags {
    //     println!("  Tag: {}, Value: {:?}", tag, value);
    // }
    // if let Some(pixel_data) = dicom_data.pixel_data {
    //     println!("  Pixel data shape: {:?}", pixel_data.shape());
    // } else {
    //     println!("  No pixel data available");
    // }
    // println!(); // Empty line for readability
    
    Ok(())
}

