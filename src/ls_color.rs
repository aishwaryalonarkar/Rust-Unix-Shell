extern crate chrono;
extern crate permissions;
extern crate libc;

use colored::Colorize;
use chrono::*;
use is_executable::IsExecutable;
use std::fs::{self};
use std::path::{PathBuf};
use std::os::unix::fs::MetadataExt;
use libc::{S_IRGRP, S_IROTH, S_IRUSR, S_IWGRP, S_IWOTH, S_IWUSR, S_IXGRP, S_IXOTH, S_IXUSR};
use std::os::unix::fs::PermissionsExt;

pub fn ls_color_main(path: String) {
    let mut collected_path = path;

    // If no arguments are given by user then converting it into default path
    if collected_path == "" {
        collected_path = String::from("./");
    }

    let paths = fs::read_dir(collected_path).unwrap();

    // Printing header for listDir command
    println!("{} \t {} \t {} {} \t {} \t {} \t {}","Permission".bold(), "Links".bold(),"User".bold(),"Group".bold(),"size".bold(),"Modified".bold(),"Name".bold());

    // Traversing through all files and folders from current path
    paths.for_each(|initial| {
        let initial = initial.unwrap();
        let path = initial.path();
        let metadata = path.metadata().unwrap();
        let file_mode = metadata.permissions().mode();
        metadata_and_print(file_mode as u32,path.clone());
    })
}

pub fn metadata_and_print(file_mode: u32, fn_path: PathBuf)  {

    // Collecting metadata about particular file or folder
    let metadata = &fn_path.metadata().unwrap();

    // Collecting separate metadata for symbolic link file
    let metadata_sym = fs::symlink_metadata(&fn_path).unwrap();

    // Fetching symbolic link path in PathBuf format
    let sym_path = fs::canonicalize(&fn_path).unwrap();

    // Generating a timestamp for latest modification
    let modified: DateTime<Local> = DateTime::from(metadata.modified().unwrap());

    // Declaring flag for checking file or directory
    let mut flag = String::from(""); 

    if metadata.is_dir() {
        flag = String::from("d");
    }

    if metadata.is_file() {
        flag = String::from("-"); 
    }

    // Calculating and decoding file permissions using "permission_checker" function
    let user_permission = permission_checker(file_mode,  S_IRUSR, S_IWUSR, S_IXUSR);
	let grpup_permission = permission_checker(file_mode,  S_IRGRP, S_IWGRP, S_IXGRP);
	let other_permission = permission_checker(file_mode,  S_IROTH, S_IWOTH, S_IXOTH);

    // Getting file name in string format ...
    let f_name = fn_path.file_name().unwrap();
    let f_name_str = f_name.to_str().unwrap();
    let file_name = String::from(f_name_str);

    if !file_name.starts_with(".") {
        let hard_link = metadata.nlink();
        let size = metadata.len();
        let symbolic = metadata_sym.file_type();
        let symbolic_size = metadata_sym.len();
        let sym_file_mode = metadata_sym.permissions().mode();
        
        if metadata.is_dir() {
            println!("{} \t {} \t {} {} \t {} \t {} \t {}",[flag.clone(),user_permission.clone(), grpup_permission.clone(), other_permission.clone()].join(""),hard_link,"root","root",size,modified.format("%_d %b %H:%M").to_string(),file_name.blue().bold());
        } else if metadata.is_file() {

            if symbolic.is_symlink() {
                
                let user_permission = permission_checker(sym_file_mode,  S_IRUSR, S_IWUSR, S_IXUSR);
                let grpup_permission = permission_checker(sym_file_mode,  S_IRGRP, S_IWGRP, S_IXGRP);
                let other_permission = permission_checker(sym_file_mode,  S_IROTH, S_IWOTH, S_IXOTH);

                println!("l{} \t {} \t {} {} \t {} \t {} \t {} -> {}",[user_permission.clone(), grpup_permission.clone(), other_permission.clone()].join(""),hard_link,"root","root", symbolic_size, modified.format("%_d %b %H:%M").to_string(),file_name.cyan(),sym_path.into_os_string().into_string().unwrap());
            } else if fn_path.is_executable() {
                    println!("{} \t {} \t {} {} \t {} \t {} \t {}",[flag.clone(), user_permission.clone(), grpup_permission.clone(), other_permission.clone()].join(""),hard_link,"root","root",size,modified.format("%_d %b %H:%M").to_string(),file_name.green().bold());
            } else if file_name.ends_with(".tar.gz") {
                println!("{} \t {} \t {} {} \t {} \t {} \t {}",[flag.clone(), user_permission.clone(), grpup_permission.clone(), other_permission.clone()].join(""),hard_link,"root","root",size,modified.format("%_d %b %H:%M").to_string(),file_name.red().bold());
            } else {
                println!("{} \t {} \t {} {} \t {} \t {} \t {}",[flag.clone(), user_permission.clone(), grpup_permission.clone(), other_permission.clone()].join(""),hard_link,"root","root",size,modified.format("%_d %b %H:%M").to_string(),file_name)
            }
        }
    }
}


pub fn permission_checker(file_mode: u32, r: u32, w: u32, x: u32) -> String {

    if file_mode & r == 0 && file_mode & w == 0 && file_mode & x == 0 {
        "---".to_string()
    } else if file_mode & w == 0 && file_mode & x == 0 {
        "r--".to_string()
    } else if file_mode & r == 0 && file_mode & x == 0 {
        "-w-".to_string()
    } else if file_mode & r == 0 && file_mode & w == 0 {
        "--x".to_string()
    } else if file_mode & w == 0 {
        "r-x".to_string()
    } else if file_mode & x == 0 {
        "rw-".to_string()
    } else if file_mode & r == 0 {
        "-wx".to_string()
    } 
    else {
        "rwx".to_string()
    }
}    


