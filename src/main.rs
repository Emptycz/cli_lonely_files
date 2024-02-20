use std::collections::HashMap;
use std::fs;
use anyhow::{Context, Result};
use std::io::{self, Write};
use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// Folder to scan for files
    input: PathBuf,
    /// Look only for extensions
    extensions: Option<Vec<String>>,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let directory_content = fs::read_dir(&args.input)
        .with_context(|| format!("Failed to read directory '{}'", args.input.display()))?;
    let mut file_extensions: HashMap<String, Vec<PathBuf>> = HashMap::new();

    println!("\n \n");
    println!("#### Scanning folder: {}", args.input.display());
    println!("#### Looking for extensions: {:?}", args.extensions);

    map_files(directory_content, &args.input, &args.extensions, &mut file_extensions)?;
    let lonely_files = check_lonely_files(&file_extensions)?;

    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);

    println!("=========================");
    println!("#### Lonely files found: \n");
    for files in lonely_files {
        writeln!(handle, "{}", files)?;
    }
    return Ok(())
}

fn map_files(
    directory: fs::ReadDir,
    base_path: &PathBuf,
    extensions: &Option<Vec<String>>,
    file_extensions: &mut HashMap<String, Vec<PathBuf>>,
) -> Result<()> {
    for (_, entry) in directory.into_iter().enumerate() {
        let unwrapped_entry = entry?;
        let file_type = unwrapped_entry.file_type()?;
        if file_type.is_dir() {
            let subdir_path = base_path.join(unwrapped_entry.file_name());
            let subdir_content = fs::read_dir(&subdir_path)
                .with_context(|| format!("Failed to read directory '{}'", subdir_path.display()))?;
            map_files(subdir_content, &subdir_path, extensions, file_extensions)?;
            return Ok(());
        }

        if file_type.is_file() {
            let file_name = unwrapped_entry.file_name();
            let extension = file_name.to_str().unwrap().split('.').last().unwrap();

            if extensions.is_some() && !extensions.as_ref().unwrap().contains(&extension.to_string()) {
                continue;
            }

            let files_with_extension = file_extensions.entry(extension.to_string()).or_default();

            let file_name_without_extensions = file_name.to_str().unwrap().split('.').next().unwrap();

            files_with_extension.push(base_path.join(file_name_without_extensions));
        }
    }
    return Ok(())
}

fn check_lonely_files(file_extensions: &HashMap<String, Vec<PathBuf>>) -> Result<Vec<String>> {
    let mut lonely_files: Vec<String> = vec![];
    let extensions = file_extensions.keys().collect::<Vec<&String>>();

    for (extension, files) in file_extensions.iter() {
        for file in files.iter() {
            let mut has_coupling = false;
            for extension_to_check in extensions.iter() {
                if *extension_to_check == extension {
                    continue;
                }

                if find_in_vector(file_extensions.get(*extension_to_check).unwrap().to_vec(), &file.to_path_buf()) {
                    has_coupling = true;
                    break;
                }
            }

            if !has_coupling {
                lonely_files.push(format!("{}.{}", file.to_str().unwrap().to_string(), extension));
            }
        }
    }

    return Ok(lonely_files);
}

fn find_in_vector(vector: Vec<PathBuf>, file: &PathBuf) -> bool {
    for file_in_vector in vector.iter() {
        if file_in_vector == file {
            return true;
        }
    }
    return false;
}
