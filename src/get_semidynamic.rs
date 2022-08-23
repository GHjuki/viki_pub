use std::collections::HashMap;
use std::fs;
use chrono::{NaiveDate};

pub fn get() -> HashMap<String, String> {
    let tmpstr = fs::read_to_string("c:/ARM/Logs/CashRegisters.log");
    let mut config: HashMap<String, String> = HashMap::new(); //итоговый хешмэп с конфигом
    match tmpstr {
        Ok(t) => {
            for line in t.lines() {
                let mut vec:Vec<String>= Vec::new();
                for word in line.split_whitespace() {vec.push(word.to_string());}
                match vec.get(0) {
                    None => {}
                    Some(t) => {
                        match &t[..] {
                            "OldestOfdDoc" => {config.insert("OldestOfdDoc".to_string(), safe_unwrap(vec.get(2)));}
                            "CountNotSendOfdDoc" => {config.insert("CountNotSendOfdDoc".to_string(),safe_unwrap(vec.get(2)));}
                            "Fiscal" => {config.insert("Fiscal".to_string(),safe_unwrap(vec.get(2)));}
                            "FR_SN" => {config.insert("FR_SN".to_string(),safe_unwrap(vec.get(2)));}
                            "INN" => {config.insert("INN".to_string(),safe_unwrap(vec.get(2)));}
                            "IsDocumentOpen"=> {config.insert("IsDocumentOpen".to_string(),safe_unwrap(vec.get(2)));}
                            "Session"=> {config.insert("Session".to_string(),safe_unwrap(vec.get(2)));}
                            "IsSessionOpen"=> {config.insert("IsSessionOpen".to_string(),safe_unwrap(vec.get(2)));}
                            "CheckNumber"=> {config.insert("CheckNumber".to_string(),safe_unwrap(vec.get(2)));}
                            "Is24"=> {config.insert("Is24".to_string(), safe_unwrap(vec.get(2)));}
                            "NoPaper"=> {config.insert("NoPaper".to_string(), safe_unwrap(vec.get(2)));}
                            "NeedContinue"=> {config.insert("NeedContinue".to_string(), safe_unwrap(vec.get(2)));}
                            "FrDate"=> {config.insert("FrDate".to_string(), safe_unwrap(vec.get(2)));}
                            "FR_NALOG_REG"=> {config.insert("FR_NALOG_REG".to_string(), safe_unwrap(vec.get(2)));}
                            "FN_SN"=> {config.insert("FN_SN".to_string(), safe_unwrap(vec.get(2)));}
                            "FR_OFD_INN"=> {config.insert("FR_OFD_INN".to_string(),safe_unwrap(vec.get(2)));}
                            "UDescription"=> {config.insert("UDescription".to_string(), safe_unwrap(vec.get(2)));}
                            "UModel"=> {config.insert("UModel".to_string(), safe_unwrap(vec.get(2)));}
                            "FrOffline"=> {config.insert("FrOffline".to_string(), safe_unwrap(vec.get(2)));}
                            "FrEndDate"=> {config.insert("FrEndDate".to_string(), safe_unwrap(vec.get(2)));}
                            "DocNumber"=> {config.insert("DocNumber".to_string(), safe_unwrap(vec.get(2)));}
                            "RomVersion"=> {config.insert("RomVersion".to_string(), safe_unwrap(vec.get(2)));}
                            "FFD"=> {config.insert("FFD".to_string(), safe_unwrap(vec.get(2)));}
                            "BatteryInfo"=> {config.insert("batLevel".to_string(), safe_unwrap(vec.get(2)));}//BatteryInfo
                            &_ => {}
                        }
                    }
                }
            }
        }
        Err(_) => {
            eprintln!("Error open CashRegisters.log");
        }
    }
// convert source data to server like data
    transform_bool(&mut config,"Fiscal");

    transform_bool(&mut config,"IsDocumentOpen");

    transform_bool(&mut config,"IsSessionOpen");

    transform_bool(&mut config,"NoPaper");
    transform_bool(&mut config,"Is24");
    transform_bool(&mut config,"NeedContinue");
    transform_bool(&mut config,"FrOnline");

    match config.get("OldestOfdDoc") {
        None => { println!("OldestOfdDoc is not in list") }
        Some(oldDoc) => { config.insert("OldestOfdDoc".to_string(),date_to_timestamp(oldDoc)); }
    }
    match config.get("FrDate") {
        None => { println!("FrDate is not in list") }
        Some(oldDoc) => { config.insert("FrDate".to_string(), date_to_timestamp(oldDoc)); }
    }
    match config.get("FrEndDate") {
        None => { println!("FrEndDate is not in list") }
        Some(oldDoc) => { config.insert("FrEndDate".to_string(), date_to_timestamp(oldDoc)); }
    }

//<get arm_config from CashRegisters.log
    config.insert("ARMversion".to_string(),arm_config());
//>
    config
}

fn safe_unwrap (s: Option<&String>) -> String {
    match s {
        Some(t) => {return t.to_string()}
        None => return "".to_string()
    }
}

fn date_to_timestamp(oldDoc:&String) -> String {
    // println!("{:?}",oldDoc.len());
    if oldDoc.len()==10 {
        let mut vec:Vec<String> = Vec::new();
        for i in oldDoc.split('.') {
            vec.push(i.to_string());
        }
        let timestamp = NaiveDate::from_ymd(vec.get(2).unwrap().parse::<i32>().unwrap(), vec.get(1).unwrap().parse::<u32>().unwrap(), vec.get(0).unwrap().parse::<u32>().unwrap()).and_hms(0,0,0).timestamp();
            return timestamp.to_string();
    }
    "".to_string()
}

fn transform_bool(map: &mut HashMap<String, String>, string:&str) {
    match map.get(string) {
        None => {}
        Some(t) => {
            match &t[..]{
                "True" => {map.insert(string.to_string(),"1".to_string());},
                "False" => {map.insert(string.to_string(),"0".to_string());},
                _ => {}
            }
        }
    }
}

fn arm_config() -> String{
    let mut ARMversion = String::new();
    let mut last_line = String::new();
    let tmpstr=fs::read_to_string("c:/ARM/Logs/YaRMarka.log");
    match tmpstr {
        Ok(t) => {
            for i in t.lines() {
                last_line=i.to_string();             //get last line from file
            }
        }
        Err(_) => {eprintln!("Can't read Yarmarka.log")}
    }

    let mut vec:Vec<String>=Vec::new();
    for i in last_line.split_whitespace() {vec.push(i.to_string())}
    match vec.get(2){
        None => {eprintln!("Can't get ARM version from Yarmarka.log")}
        Some(t) => {ARMversion=t.to_string()}

    }
    ARMversion
}
