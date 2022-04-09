
use std::fs;
use std::path::Path;
use std::error::Error;
use std::process;


pub fn tree_display(path : String) {
    let mut dir_path = path;
    if dir_path == "" {
        dir_path = String::from("./");
    }
    println!("{}",dir_path);

    // let mut disp_vec : Vec<Vec<String>> = Vec::new();
	if let Err(ref e) = run(Path::new(&dir_path),0,Vec::new(),&dir_path) {
		println!("{:?}", e);
		process::exit(1);
	}
    println!("");
}

fn run(dir: &Path, mut level : usize,  mut vec : Vec<String>, dir_path:&String) -> Result<(), Box<dyn Error>> {

	if dir.is_dir() {
        level = level + 1;
		for entry in fs::read_dir(dir)? {
				let entry = entry?;
				let file_name = entry.file_name().into_string().or_else(|f| Err(format!("Invalid entry: {:?}", f)))?;
				if file_name.chars().nth(0).unwrap() != '.' {

                    // level = depth from directory
                    for _i in 1..level {
                        print!("    ");
                    }
                    print!("|");
                
                    println!("__{}", file_name);

                    let s1 = "/".to_string();
                    let s2 = file_name.clone();
                    // contains file_name for current directory 
                    let mut s3 = String::new();
                    // contains entire path
                    let mut s4 = String::new();
                    s3 += &s2;
                    s3 += &s1;
                    vec.push(s3.clone());
                    s4+=dir_path;
                    s4+="/";
                    for i in vec.iter() {
                        s4 += i;
                    }

                    if let Err(ref e) = run(Path::new(&s4),level,vec.clone(),dir_path) {
                            println!("{:?}", e);
                            process::exit(1);
                    }
                    vec.pop();
                }
		}
	}

	Ok(())
    
}

pub fn list_all(path : String) {
    let mut dir_path = path;
    if dir_path == "" {
        dir_path = String::from("./");
    }
    println!("{}",dir_path);
    let mut disp_vec : Vec<String> = Vec::new();

	if let Err(ref e) = run_all(Path::new(&dir_path),&mut disp_vec) {
		println!("{}", e);
		process::exit(1);
	}
    let mut max = 0;
    let mut screen_max = 5;
    for i in disp_vec.iter() {
        if i.len() > max {
            max = i.len();
        }
    }
    if max>17 {
        screen_max = 3;
    }
    let mut count = 0;
    for i in disp_vec.iter() {
        print!("{}",i);
        count+=1;
        if i.len() < max {
            for _k in i.len()..max {
                print!(" ");
            }
        }
        print!("\t");
        if count%screen_max == 0 {
            println!("");
        }
    }
    println!("");
}

fn run_all(dir: &Path, vec : &mut Vec<String>) -> Result<(), Box<dyn Error>> {
	if dir.is_dir() {
		for entry in fs::read_dir(dir)? {
				let entry = entry?;
				let file_name = entry.file_name().into_string().or_else(|f| Err(format!("Invalid entry: {:?}", f)))?;
                vec.push(file_name);
		}
	}
	Ok(())
}