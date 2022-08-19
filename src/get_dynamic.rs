mod get_html;


use std::collections::HashMap;
use std::fs;
// use std::io::BufRead;
use std::time::{SystemTime};
use super::*;

pub fn get(utm:&String) ->HashMap<String,String> {
    let mut config:HashMap<String,String> = HashMap::new();
//< get uptime
    match uptime_lib::get() {
        Ok(uptime) => {
            config.insert("uptime".to_string(),uptime.as_millis().to_string());
        }
        Err(err) => {
            eprintln!("uptime: {}", err);
            config.insert("uptime".to_string(),"0".to_string());
        }
    }
//>

//<  get current time -pipotime
    let time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    config.insert("pipotime".to_string(),time.to_string());
//>

//< get freespace
    match fs2::free_space("c:/") {
        Ok(t) => {config.insert("freespace".to_string(),(t/1000000).to_string());}
        Err(_) => {config.insert("freespace".to_string(),"0".to_string());}
    }
//>


//<get html
    match get_html::get(&utm) {
        Ok(t) => {
            // println!("{:?}",t)
            add(&mut config,t);
        }
        Err(_) => {eprintln!("Cant read utm page - {}",utm)}
    };
//>

// < sending supportRequest if exist;
    config.insert("supportReq".to_string(), get_support());
//>

    config
}
// fn arm_config() -> String{
//     let mut ARMversion = String::new();
//     let mut last_line = String::new();
//     let tmpstr=fs::read_to_string("c:/ARM/Logs/YaRMarka.log");
//     match tmpstr {
//         Ok(t) => {
//             for i in t.lines() {
//                 last_line=i.to_string();             //get last line from file
//             }
//         }
//         Err(_) => {eprintln!("Can't read Yarmarka.log")}
//     }
//
//     let mut vec:Vec<String>=Vec::new();
//     for i in last_line.split_whitespace() {vec.push(i.to_string())}
//     match vec.get(2){
//         None => {eprintln!("Can't get ARM version from Yarmarka.log")}
//         Some(t) => {ARMversion=t.to_string()}
//
//     }
//     ARMversion
// }
pub fn get_support() -> String {
    let tmp_str = fs::read_to_string("c:/ARM/Logs/SupportRequest.txt");
    match tmp_str {
        Ok(s) => {
            // println!("support - {:?}",s);
            return s;
        }
        Err(e) => {println!("Error on reading SupportRequest.txt: {:?}",e)}
    }
    "".to_string()
}