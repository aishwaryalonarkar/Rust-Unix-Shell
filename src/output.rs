use std::fs::File;
use std::io::Write;

pub fn write_output(output: &str, data : String) {
    // println!(" write output ");\
    let mut file = match File::create(output) {
        Err(why) => panic!("couldn't create  {}", why),
        Ok(file) => file,
    };
    match file.write_all(data.as_bytes()) {
        Err(why) => panic!("couldn't write to  {}", why),
        Ok(_) => println!("successfully wrote to "),
    }
}   