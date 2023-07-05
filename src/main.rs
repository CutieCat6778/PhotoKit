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
        .arg(arg!(list: -l --list <LIST> "The path of the copy list."))
}

fn main() {
    // Get command line arguments
    let matches = cli().get_matches();
    // Parse the arguments to strings
    let mut from = matches
        .get_one::<String>("from")
        .expect("The <FROM> argument cannot be processed!").to_string();
    let mut to = matches
        .get_one::<String>("to")
        .expect("The <TO> argument cannot be processed!")
        .to_string();
    let list = matches
        .get_one::<String>("list")
        .expect("The <LIST> argument cannot be processed!");
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
    // Scan the given dir for files
    let path_list = scan_dir(path, list_path);
    copy_files(path_list, to, from).expect("Failed to copy files")
}

// Scan dir, getting the file path
/*
    Input: Directory path
    Output: List of file path
    Function: Scan for list file and let user chose what to copy
*/
fn scan_dir(dir: &Path, list: &Path) -> Vec<String> {
    let mut result: Vec<String> = vec![String::new(); 0];

    let list_file = read_file(list);
    let allowed_files: HashSet<&str> = list_file.lines().collect(); 

    // Visit dir and check if it is a dir
    if dir.is_dir() {
        let entries = fs::read_dir(dir)
            .expect("The original directory cannot be read!")
            .map(|a| {
                a.expect("The files path in original directory cannot the read!")
                    .path()
            })
            .filter(|a| a.display().to_string().ends_with(".txt"));
        for entry in entries {
            let file_result = entry.display();
            let file_path = file_result.to_string();
            let file_name = file_path.split("/").last().unwrap().to_string();
            let file_sequence = format_file_name(&file_name);
            if allowed_files.contains(&file_sequence.as_str()) {
                result.insert(result.len(), file_name);
            } else {
                println!("Not allow {}", &file_sequence);
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

fn read_file(path: &Path) -> String {
    let contents = fs::read_to_string(path);

    let contents = match contents {
        Ok(contents) => contents,
        Err(error) => panic!("Failed to read file list {:?}", error)
    };

    contents.to_string()
}

fn format_file_name(name: &String) -> String {
    let pattern = r".*_(\d+)[^\d]\w+";
    let re = Regex::new(pattern);

    let re = match re {
        Ok(re) => re,
        Err(error) => panic!("Failed to initialize regex {:?}", error)
    };

    if let Some(captures) = re.captures(&name) {
        if let Some(sequence_number) = captures.get(1) {
            let sequence_number = sequence_number.as_str().parse::<u32>().unwrap();
            return sequence_number.to_string();
        }
    } else {
        return String::new()
    };

    return String::new()
}
