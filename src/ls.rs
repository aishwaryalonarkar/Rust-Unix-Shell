extern crate chrono;
extern crate permissions;
extern crate libc;

use chrono::*;
use std::fs::{self};
use std::path::{PathBuf};
use std::os::unix::fs::MetadataExt;
use libc::{S_IRGRP, S_IROTH, S_IRUSR, S_IWGRP, S_IWOTH, S_IWUSR, S_IXGRP, S_IXOTH, S_IXUSR};
use std::os::unix::fs::PermissionsExt;

pub fn ls_main() {
    let collect_string = "test5";

    // Collecting path
    let pass_path = String::from("./") + &collect_string;
    let paths = fs::read_dir(pass_path).unwrap();

    // Printing header for listDir command
    println!("{} \t {} \t {} {} \t {} \t {} \t {}","Permission", "Links", "User", "Group", "size", "Modified", "Name");

    paths.for_each(|initial| {
        let initial = initial.unwrap();
        let path = initial.path();
        let metadata = path.metadata().unwrap();
        let file_mode = metadata.permissions().mode();
        
        metadata_and_print(file_mode as u32,path.clone());
    })
}

pub fn metadata_and_print(file_mode: u32, fn_path: PathBuf)  {

    let metadata = &fn_path.metadata().unwrap();
    let metadata_sym = fs::symlink_metadata(&fn_path).unwrap();
    let sym_path = fs::canonicalize(&fn_path).unwrap();
    let modified: DateTime<Local> = DateTime::from(metadata.modified().unwrap());
    let mut flag = String::from(""); 

    if metadata.is_dir() {
        flag = String::from("d");
    }

    if metadata.is_file() {
        flag = String::from("-"); 
    }

    let user_permission = permission_checker(file_mode,  S_IRUSR, S_IWUSR, S_IXUSR);
	let grpup_permission = permission_checker(file_mode,  S_IRGRP, S_IWGRP, S_IXGRP);
	let other_permission = permission_checker(file_mode,  S_IROTH, S_IWOTH, S_IXOTH);

    let f_name = fn_path.file_name().unwrap();
    let f_name_str = f_name.to_str().unwrap();
    let file_name = String::from(f_name_str);

    if !file_name.starts_with(".") {
        let hard_link = metadata.nlink();
        let size = metadata.len();
        let symbolic = metadata_sym.file_type();
        let symbolic_size = metadata_sym.len();
        let sym_file_mode = metadata_sym.permissions().mode();

        if symbolic.is_symlink() {

            let user_permission = permission_checker(sym_file_mode,  S_IRUSR, S_IWUSR, S_IXUSR);
            let grpup_permission = permission_checker(sym_file_mode,  S_IRGRP, S_IWGRP, S_IXGRP);
            let other_permission = permission_checker(sym_file_mode,  S_IROTH, S_IWOTH, S_IXOTH);

            println!("l{} \t {} \t {} {} \t {} \t {} \t {} -> {}",[user_permission.clone(), grpup_permission.clone(), other_permission.clone()].join(""),hard_link,"root","root", symbolic_size, modified.format("%_d %b %H:%M").to_string(),file_name,sym_path.into_os_string().into_string().unwrap());
        } else {
            println!("{} \t {} \t {} {} \t {} \t {} \t {}",[flag.clone(),user_permission.clone(), grpup_permission.clone(), other_permission.clone()].join(""),hard_link,"root","root",size,modified.format("%_d %b %H:%M").to_string(),file_name);
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


