use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::ptr::hash;

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
                        let d=vec.get(2);
                        match &t[..] {
                            "OldestOfdDoc" => {config.insert("OldestOfdDoc".to_string(), safe_unwrap(vec.get(2)));}
                            "CountNotSendOfdDoc" => {config.insert("CountNotSendOfdDoc".to_string(),safe_unwrap(vec.get(2)));}
                            "Fiscal" => {config.insert("Fiscal".to_string(),safe_unwrap(vec.get(2)));}
                            "FR_SN" => {config.insert("GR_SN".to_string(),safe_unwrap(vec.get(2)));}
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
                            "BatteryInfo"=> {config.insert("BatteryInfo".to_string(), safe_unwrap(vec.get(2)));}
                            &_ => {}
                        }
                    }
                }
            }
        }
        Err(_) => {
            eprintln!("Error open client.ovpn");
        }
    }

    config
}

fn safe_unwrap (s: Option<&String>) -> String {
    match s {
        Some(T) => {return T.to_string()}
        None => return "".to_string()
    }
}