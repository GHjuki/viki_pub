use std::collections::HashMap;
use std::fs;

//<get priceDate
pub fn get() -> HashMap<String,String> {
    let mut config:HashMap<String,String> = HashMap::new();
    let tmpstr = fs::read_to_string("c:/ARM/Logs/LastPrice.info");
    let mut beg_date = String::new();
    match tmpstr {
        Ok(t) => {
            for i in t.lines() {
                // println!("{:?}",i);
                if i.contains("beg_date") {
                    config.insert("priceDate".to_string(),i[11..21].to_string());
                    break;
                }
            }
        }
        Err(_) => { eprintln!("Can't open LastPrice.info") }
    }
    println!("{}", beg_date); // - priceDate
//>

// get target
    let tmpstr = fs::read_to_string("c:/Program files/OpenVPN/Config/client.ovpn");
    match tmpstr {
        Ok(t) => {
            for line in t.lines() {
                if line.contains("cert") & line.contains(".crt") {
                    let tmpvec1: Vec<&str> = line.split_whitespace().collect();
                    let tmpvec = tmpvec1.get(1).unwrap();
                    let tmpvec2: Vec<&str> = tmpvec.split('.').collect();
                    config.insert("target".to_string(),tmpvec2.get(0).unwrap().to_string()); // -target
                }
            }
        }
        Err(_) => {
            eprintln!("Error open client.ovpn");
            config.insert("target".to_string(),"666".to_string());
        }
    }
    config
//>
}

pub fn get_config(pingtime: &mut u64, utm: &mut String){
    // let mut pingtime:u64=60000;
    // let mut utm:String = String::from("http://127.0.0.1:8080/home");
    let tmpstr = fs::read_to_string("c:/distrib/vikionline/vikionline.conf");
//    println!("{:?}",tmpstr);
    match tmpstr {
        Ok(t) => {
            let mut tmp_vec:Vec<String> = Vec::new();
            for i in t.split("#") {
                tmp_vec.push(i.to_string());
            }
            for i in 0..tmp_vec.len() {
                if tmp_vec[i].contains("pingtime") {
                    if i+1>=tmp_vec.len() {
                        eprintln!("Wrong format of vikionlie.conf");
                        break;
                    }
                    *pingtime=tmp_vec[i+1].parse().unwrap();
                } else if tmp_vec[i].contains("utm") {
                    if i+1>=tmp_vec.len() {
                        eprintln!("Wrong format of vikionlie.conf");
                        break;
                    }
                    *utm=tmp_vec[i+1].to_string();
                    utm.push_str("/home");
                }
            }
        }
        Err(_) => {println!("Can't find vikionline.conf")}
    };
    // (pingtime,*utm)
}
