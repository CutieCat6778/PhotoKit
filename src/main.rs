use clap::{arg, Command};
use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

// Initializing the cli application

fn cli() -> clap::Command {
    Command::new("photokit")
        .about("A cli-software to help photographers copy their photography files to their work folder with a sorted list.")
        .arg_required_else_help(true)
        .arg(arg!(from: -f --from <FROM> "The path of the original folder."))
        .arg(arg!(to: -t --to <TO> "The path of the target folder."))
        .arg(arg!(to: -e --ext <EXT> "The extension name of the files"))
        .arg(arg!(list: -l --list <LIST> "The path of the copy list."))
}

fn main() {
    // Get command line arguments
    let matches = cli().get_matches();
    // Parse the arguments to strings
    // Origin folder path
    let mut from = matches
        .get_one::<String>("from")
        .expect("The <FROM> argument cannot be processed!").to_string();
    // Target folder path
    let mut to = matches
        .get_one::<String>("to")
        .expect("The <TO> argument cannot be processed!")
        .to_string();
    // Extention name
    let ext = matches
        .get_one::<String>("ext")
        .expect("The <EXT> argument cannot be processed!");
    // Filename list file path
    let list = matches
        .get_one::<String>("list")
        .expect("The <LIST> argument cannot be processed!");
    // Check if the folder path doesn't ends with "/" then add it
    let to: String = if !to.ends_with("/") {
        to.push('/');
        to
    } else {
        to
    };
    let from: String = if !from.ends_with("/") {
        from.push('/');
        from
    } else {
        from
    };
    // Creating the path
    let path = Path::new(&from);
    let list_path = Path::new(list);
    // Scan the given dir for files and filter the unessesary files
    let path_list = scan_dir(path, list_path, ext);
    // Copy the files to the target folder
    copy_files(path_list, to, from).expect("Failed to copy files")
}

// Scan dir, getting the file path
/*
    Input: Directory path, List path, ext name
    Output: List of file path
    Function: Scan for list file and let user chose what to copy
*/
fn scan_dir(dir: &Path, list: &Path, ext: &String) -> Vec<String> {
    // Create a array, that contains all the filenames
    let mut result: Vec<String> = vec![String::new(); 0];

    // Get the filename lists
    let list_file = read_file(list);
    // Convert the filename lists to an array
    let allowed_files: HashSet<&str> = list_file.lines().collect(); 

    // Visit dir and check if it is a dir
    if dir.is_dir() {
        // Get the original folder
        let entries = fs::read_dir(dir)
            .expect("The original directory cannot be read!")
            .map(|a| {
                a.expect("The files path in original directory cannot the read!")
                    .path()
            })
            .filter(|a| a.display().to_string().ends_with(ext)); // Filter files with the correct filename, so it can reduce performance usage. (Smaller loops)
        // Loop through all the files and check if the filename has the same sequence
        for entry in entries {
            let file_result = entry.display();
            let file_path = file_result.to_string();
            let file_name = file_path.split("/").last().unwrap().to_string();
            // Format the file name
            let file_sequence = format_file_name(&file_name);
            // Check if contains
            if allowed_files.contains(&file_sequence.as_str()) {
                // If it is true, then add it into the copy list.
                result.insert(result.len(), file_name);
            }
        }
    };

    result
}

// Copy a selected list of files
/*
    Input: List of file dir, target directory
    Output: None
    Function: Copy files from the files_list to the taprget_dir
*/
fn copy_files(files_list: Vec<String>, target_dir: String, origin_dir: String) -> std::io::Result<()> {
    for file in files_list {
        let new_path = format!("{target_dir}{file}");
        let origin_path = format!("{origin_dir}{file}");
        fs::copy(origin_path, new_path)?;
    }
    Ok(())
}

// Read the contents of the filename list
/*
    Input: list path
    Output: list of files
*/
fn read_file(path: &Path) -> String {
    let contents = fs::read_to_string(path);

    let contents = match contents {
        Ok(contents) => contents,
        Err(error) => panic!("Failed to read file list {:?}", error)
    };

    contents.to_string()
}

// Format the file name to sequence numbers
/*
    Input: filename
    Output: file sequence
*/
fn format_file_name(name: &String) -> String {
    // Pattern regex, got it from ChatGPT :)
    let pattern = r".*_(\d+)\.\w+|(\d+)\.\w+";
    let re = Regex::new(pattern);

    let re = match re {
        Ok(re) => re,
        Err(error) => panic!("Failed to initialize regex {:?}", error)
    };

    if let Some(captures) = re.captures(&name) {
        if let Some(sequence_number) = captures.get(1).or(captures.get(2)) {
            let sequence_number_str = sequence_number.as_str();
            let sequence_number = u64::from_str_radix(sequence_number_str, 10).unwrap();
            return sequence_number.to_string() // Output: 321
        }
    } else {
        return String::new()
    };

    return String::new()
}
