
mod get_static;
mod get_dynamic;
mod get_semidynamic;

fn main() {
    struct Config {
        pingtime:u64,
        utm:String,
    }
    // < > Set default value of pingtime and utm address
    // let mut config:Config = Config { pingtime: 60000, utm: "http://127.0.0.1:8080/home".to_string() };
    let mut config:Config = Config { pingtime: 10000, utm: "http://172.16.23.47:8080/home".to_string() };
    get_static::get_config(&mut config.pingtime,&mut config.utm);
    println!("{}-{}",config.pingtime,config.utm);
    const VERSION: &str = env!("CARGO_PKG_VERSION"); // версия программы из toml -version
    println!("{:#?}",get_static::get());
    println!("{:#?}",get_dynamic::get(&config.utm));

}