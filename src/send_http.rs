use std::collections::HashMap;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::ptr::write;
use std::time::SystemTime;
use chrono::{DateTime, Local, Utc};
// use std::intrinsics::float_to_int_unchecked;
use super::get_dynamic;

pub fn send(config: HashMap<String,String>) -> String{
    // let response = ureq::get("http://172.16.16.1/piposadder.php");
    // let vec = vec![("1111","1111111"),("2222","22222222")];
    // let arr = [("3333","333333333"),("4444","4444444444")];
    // let vec1 = Vec::from_iter(config.iter());
    let mut vec : Vec<(&str,&str)> = Vec::new();
    for item in &config {
        vec.push((item.0.as_str(),item.1.as_str()));
    }
    // println!("{:?}",vec1);
    let response = ureq::post("http://172.16.16.1/piposadder.php")
        .send_form(&vec) ;  // add ?foo=bar+baz
        // .unwrap();
    match response {
        Ok(r) => {
            // println!("{:?}",r.into_string());
            /// now checking answer to understand is support request done?
            if r.into_string().unwrap_or_else(|e| {format!("Cant unwrap answer for post request - {e}")}).contains("support_done")
                && Path::new("c:/ARM/Logs/SupportRequest.txt").exists() {
                // println!("EEEEEE");
                work_with_sup_req();

            }
        }
        Err(e) => {println!("{:?}",e)}
    }
    "".to_string()
}
fn work_with_sup_req() {
    if !Path::new("c:/ARM/Logs/supportLog.log").exists() {
        let new_file = File::create("c:/ARM/Logs/supportLog.log");
        if let Err(e) = new_file {
            println!("Error on creating supportLog.log: {}", e)
        };
    }
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("c:/ARM/Logs/supportLog.log");
    match file {
        Ok(mut f) => {
            let support = get_dynamic::get_support();
            let system_time = SystemTime::now();
            let datetime: DateTime<Local> = system_time.into();
            println!("{}", datetime.format("%d.%m.%Y %T"));
            if let Err(e) = f.write_all("-------------------------------------------------------\n\r".as_bytes()) {
                eprintln!("Couldn't write to SupportRequest.txt: {}", e);
            }
            if let Err(e) = f.write_all(datetime.format("%d.%m.%Y %T \n\r").to_string().as_bytes()) {
                eprintln!("Couldn't write to SupportRequest.txt: {}", e);
            }
            if let Err(e) = f.write_all(support.as_bytes()) {
                eprintln!("Couldn't write to SupportRequest.txt: {}", e);
            }
            if let Err(e) = fs::remove_file("c:/ARM/Logs/SupportRequest.txt") {
                println!("Can't remove SupportRequest.txt")
            }
        }
        Err(_) => { println!("Can't open to append supportLog.log") }
    }
}