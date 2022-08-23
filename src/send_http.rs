use std::collections::HashMap;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
// use std::ptr::write;
use std::time::SystemTime;
use chrono::{DateTime, Local};
// use std::intrinsics::float_to_int_unchecked;
use super::get_dynamic;

pub fn send(config: HashMap<String,String>) -> String{
    let mut vec : Vec<(&str,&str)> = Vec::new();
    for item in &config {
        vec.push((item.0.as_str(),item.1.as_str()));
    }
    let response = ureq::post("http://172.16.16.1/piposadder.php")
        .send_form(&vec) ;  // add ?foo=bar+baz
    match response {
        Ok(r) => {
            if r.into_string().unwrap_or_else(|e| {format!("Cant unwrap answer for post request - {e}")}).contains("support_done")
                && Path::new("c:/ARM/Logs/SupportRequest.txt").exists() {
                work_with_sup_req();
            }
        }
        Err(e) => {eprintln!("{:?}",e)}
    }
    "".to_string()
}
fn work_with_sup_req() {
    if !Path::new("c:/ARM/Logs/supportLog.log").exists() {
        let new_file = File::create("c:/ARM/Logs/supportLog.log");
        if let Err(e) = new_file {
            eprintln!("Error on creating supportLog.log: {}", e)
        };
    }
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("c:/ARM/Logs/supportLog.log");
    match file {
        Ok(mut f) => {
            let support = get_dynamic::get_support();
            let system_time = SystemTime::now();
            let datetime: DateTime<Local> = system_time.into();
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
                eprintln!("Can't remove SupportRequest.txt: {}",e)
            }
        }
        Err(_) => { eprintln!("Can't open to append supportLog.log") }
    }
}
