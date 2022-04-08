use std::fs::{self, ReadDir};
use std::io::Error;
use std::path::Path;
use std::{env, io, path::PathBuf};
use std::ffi::OsStr;

pub fn rmxn(command: String) {
    
    let command = command;

    if command.starts_with("rmallexn"){

        let chunks: Vec<_> = command.split_whitespace().collect();

        if chunks[0] == String::from("rmallexn"){

            //println!("{:?}", chunks);
            
            if chunks.len() < 2{
                println!("rmallexn missing operand");
            }
            else if chunks.len() > 2{
                println!("rmallexn takes only one parameter; multiple parameters passed");
            }
            else{
                
                let current_dir = env::current_dir();
                let current_dir = current_dir.expect("Executable must be in some directory");
                //println!("current dir {:?}",current_dir);
                    
                let dir = chunks[1];
                
                if Path::new(&dir).exists(){

                    let srcdir = PathBuf::from(&dir);
                    let absolute_path = fs::canonicalize(&srcdir).expect("Cannot resolve directory");
                    //println!("absolute path is {:?}", absolute_path);

                    let parent_absolute_path = absolute_path.parent().unwrap();
                    //println!("parent absolute path is {:?}", parent_absolute_path);
                    

                    let paths = fs::read_dir(parent_absolute_path).unwrap();
                    for path in paths {
                        //println!("Name: {}", path.unwrap().path().display());
                        let path_string = path.unwrap().path();

                        if path_string != absolute_path{

                            if Path::new(&path_string).is_file(){
                                
                                //println!("path is file");
                                //println!("Name: {:?}", path_string);
                                fs::remove_file(path_string).expect("Failed to remove a file");
                                
                            }
                            else{
                            
                                //println!("path is dir");
                                //println!("Name: {:?}", path_string);
                                let delete_dir = fs::read_dir(path_string.clone());
                                delete_dir_contents(delete_dir);
                                fs::remove_dir(path_string);
                    
                            }
                        }
                    }

                }
                else{
                    println!("path doesn't exist");
                }

            }
        }
        else{
            println!("incorrect keyword passed, try rmallexn instead");

        }

    }
}



fn delete_dir_contents(read_dir_res: Result<ReadDir, Error>) {
    if let Ok(dir) = read_dir_res {
        for entry in dir {
            if let Ok(entry) = entry {
                let path = entry.path();

                if path.is_dir() {
                    fs::remove_dir_all(path).expect("Failed to remove a dir");
                } else {
                    fs::remove_file(path).expect("Failed to remove a file");
                }
            };
        }
    };
}
