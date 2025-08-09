use clap::{Parser, Subcommand};
use std::path::PathBuf;
use rayon::prelude::*;
use indicatif::{ProgressBar, ProgressStyle};

mod consts;
mod os_utils;
use os_utils::get_dicom_paths_from_folder;
mod dicom_utils;
use dicom_utils::{read_dicom_file, extract_dicom_data, DicomData};


fn process_single_dicom(path: &PathBuf) -> Result<DicomData, Box<dyn std::error::Error + Send + Sync>> {
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
    Ok(dicom_data)
}

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
            println!("Processing DICOM files in: {}", read_args.path.display());
            
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
            let total_files = dicom_paths.len();
            println!("Found {} DICOM files to process", total_files);
            
            // Create a progress bar
            let progress_bar = ProgressBar::new(total_files as u64);
            progress_bar.set_style(
                ProgressStyle::default_bar()
                    .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len}")
                    .unwrap()
                    .progress_chars("#>-")
            );
            
            // Process files in parallel
            let results: Vec<Result<DicomData, Box<dyn std::error::Error + Send + Sync>>> = dicom_paths
                .into_par_iter()
                .map(|path| {
                    let result = process_single_dicom(&path);
                    
                    // Update progress bar
                    progress_bar.inc(1);
                    
                    result
                })
                .collect();
            
            // Finish the progress bar
            progress_bar.finish_with_message("Processing complete!");
            
            // Report any errors that occurred during processing
            let errors: Vec<_> = results.into_iter().filter_map(|r| r.err()).collect();
            if !errors.is_empty() {
                println!("\nEncountered {} errors during processing:", errors.len());
                for error in errors {
                    eprintln!("Error: {}", error);
                }
            } else {
                println!("\nAll DICOM files processed successfully!");
            }
            
            println!("Processing completed. Total files: {}", total_files);
        }
    }
    Ok(())
}


