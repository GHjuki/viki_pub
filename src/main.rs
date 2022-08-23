#![allow(non_snake_case)]
#![windows_subsystem = "windows"]
extern crate core;

mod get_static;
mod get_dynamic;
mod get_semidynamic;
mod tray_icon;
mod send_http;

use std::collections::HashMap;
use std::fs;
use std::time::SystemTime;
use eventual::Timer;

struct Config {
    pingtime:u32,
    utm:String,
}

fn main() {

    // < > Set default value of pingtime and utm address
    let mut config:Config = Config { pingtime: 60000, utm: "http://127.0.0.1:8080/home".to_string() };
   get_static::get_config(&mut config.pingtime,&mut config.utm);
   // println!("{}-{}",config.pingtime,config.utm);

   std::thread::spawn(move||launch_timer(config));
    tray_icon::launch_tray();
}

fn launch_timer(config:Config ) {
    let mut static_hash_map:HashMap<String,String> = HashMap::new();
    let mut semidynamic_hash_map:HashMap<String,String> = HashMap::new();
    let timer = Timer::new();
    const VERSION: &str = env!("CARGO_PKG_VERSION"); // версия программы из toml -version

    static_hash_map.insert("version".to_string(),VERSION.to_string());
    static_hash_map.insert("k".to_string(),"pass".to_string());
    //  println!("{:#?}",get_static::get());
    add(&mut static_hash_map,get_static::get());

    let mut tf1:SystemTime = SystemTime::now(); let mut tf2:SystemTime=SystemTime::now();

    let ticks = timer.interval_ms(config.pingtime).iter();
    for _ in ticks {
        let mut target:HashMap<String,String> = HashMap::new();
        add(&mut target,static_hash_map.clone());

        add(&mut target,get_dynamic::get(&config.utm));

        let m1= get_file_modified("c:/ARM/Logs/CashRegisters.log");
        let m2= get_file_modified("c:/ARM/Logs/YaRMarka.log");
        if m1!=tf1 || m2!=tf2 {
            add(&mut semidynamic_hash_map, get_semidynamic::get());
            tf1=m1;tf2=m2;
        }
        add(&mut target, semidynamic_hash_map.clone());
        println!("config:\n\r{:#?}",target);
        send_http::send(target);
    }
}

fn add (map : &mut HashMap<String,String>, second: HashMap<String,String>) {
    for item in second {
        map.insert(item.0,item.1);
    };

}

fn get_file_modified(f: &str) ->SystemTime{
    let f1 = fs::metadata(f);
    match f1 {
        Ok(f) => {
            if let Ok(time1) = f.modified() { return time1; }
        }
        Err(_) => {}
    }
    SystemTime::now()
}