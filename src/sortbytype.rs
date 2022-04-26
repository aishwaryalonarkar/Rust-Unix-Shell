use std::fs::{self};
use std::path::{PathBuf, Path};
use std::io::ErrorKind;
use pbr::ProgressBar;
use std::{thread, time};

pub fn sort_by_type_main(file_type : &str, dest : String) {
    let user_path = String::from("./");
    let paths = fs::read_dir(user_path).unwrap();
    let mut count = 0;
    let mut flag = true;

    let b: bool = Path::new(&dest.clone()).is_dir();
    if b {

    } else {

        fs::create_dir(&dest).unwrap_or_else(|error| {
            if error.kind() == ErrorKind::AlreadyExists {
                println!("Folder already exists! 😐");
                flag = false;
            } else {
                println!("Something went wrong! Please try again ...");
            }
        });
    }


    paths.for_each(|initial| {
        let initial = initial.unwrap();
        let path = initial.path();
        if flag == true {
            count = metadata_and_print(path.clone(), &dest, &file_type, count);
        }
    });

    if count == 0 && flag == true {
        println!("There are no matching files with given extension 😐");
        println!("Quitting... 🚫");
        return;
    }
}

pub fn metadata_and_print(fn_path: PathBuf, dest: &str, file_type: &str, count: i32) -> i32{

        let metadata = &fn_path.metadata().unwrap();
        let f_name = fn_path.file_name().unwrap();
        let f_name_str = f_name.to_str().unwrap();
        let file_name = String::from(f_name_str);
        // let bar_use = file_name.clone();
        let mut temp = count;

        if file_name.ends_with(&file_type) && metadata.is_file() {
            let destination = dest.to_owned() + &String::from("/") + &String::from(&file_name);
            // fs::copy(&file_name, destination).unwrap();
            // fs::remove_file(&file_name).unwrap(); 
            match fs::copy(&file_name, &destination) {
                Ok(_) => {},
                Err(why) => {panic!("Error : Sort by Name {}", why)}
            };
            match fs::remove_file(&file_name) {
                Ok(_) => {},
                Err(why) =>{ panic!("Error : Sort by Name {}", why)}
            };
            temp = temp + 1;
            let t_count = 100;
            let mut pb = ProgressBar::new(t_count);
            pb.format("=>");
            for _ in 0..t_count {
                pb.inc();
                thread::sleep(time::Duration::from_millis(1));
            }
            pb.finish_print("Successfully Moved!");
        } 
    temp
}