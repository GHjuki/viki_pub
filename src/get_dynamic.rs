mod get_html;


use std::collections::HashMap;
use std::fs;
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

pub fn get_support() -> String {
    let tmp_str = fs::read_to_string("c:/ARM/Logs/SupportRequest.txt");
    match tmp_str {
        Ok(s) => {
            return s;
        }
        Err(e) => {println!("Error on reading SupportRequest.txt: {:?}",e)}
    }
    "".to_string()
}