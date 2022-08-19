extern crate core;

mod get_static;
mod get_dynamic;
mod get_semidynamic;
mod tray_icon;
mod send_http;

use std::collections::HashMap;
// extern crate eventual;
use eventual::Timer;


fn main() {
    let mut target:HashMap<String,String> = HashMap::new();
    struct Config {
        pingtime:u32,
        utm:String,
    }
    // < > Set default value of pingtime and utm address
    // let mut config:Config = Config { pingtime: 60000, utm: "http://127.0.0.1:8080/home".to_string() };
    let mut config:Config = Config { pingtime: 10000, utm: "http://172.16.17.10:8080/home".to_string() };
   get_static::get_config(&mut config.pingtime,&mut config.utm);
   println!("{}-{}",config.pingtime,config.utm);

       const VERSION: &str = env!("CARGO_PKG_VERSION"); // версия программы из toml -version
    target.insert("version".to_string(),VERSION.to_string());

   //  println!("{:#?}",get_static::get());
    add(&mut target,get_static::get());

   //  println!("{:#?}",get_dynamic::get(&config.utm));
    add(&mut target,get_dynamic::get(&config.utm));

   // println!("{:#?}",get_semidynamic::get());
    add(&mut target,get_semidynamic::get());
    println!("config:\n\r{:#?}",target);

   std::thread::spawn(move||launch_timer(config.pingtime));
    tray_icon::launch_tray();
//    target = get_static::get();
    target.insert("k".to_string(),"324012".to_string());
    // send_http::send(target);
}

fn launch_timer(t:u32 ) {
    let timer = Timer::new();
    let ticks = timer.interval_ms(t).iter();
    for _ in ticks {
        println!("Timer!")
    }
}

fn add (map : &mut HashMap<String,String>, second: HashMap<String,String>) {
    for item in second {
        map.insert(item.0,item.1);
    };

}