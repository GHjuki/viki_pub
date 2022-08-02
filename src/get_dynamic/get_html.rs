use std::collections::HashMap;
use std::error::Error;

pub fn get(utm:&String) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut config: HashMap<String, String> = HashMap::new(); //итоговый хешмэп с конфигом
//    let mut resp = reqwest::blocking::get("http://172.16.17.65:8080/home")?.text()?; //for test
    let mut resp = reqwest::blocking::get(utm)?.text()?;

// < error 403
    if resp.contains("HTTP ERROR 403") {
        config.insert("UTMstatus".to_string(), "HTTP ERROR 403".to_string());
    } else {
//>
        resp.truncate(6701); // Отрезаем лишний конец
        let words: Vec<&str> = resp.split_whitespace().collect(); //весь ответ помещаем в вектор разделяя по пробелу
//    println!("{:#?}",words);
//         let mut item: usize = 0;
        let mut start = 100;

// <get проблемы с RSA
        if resp.contains("Проблемы с RSA") {
            config.insert("UTMstatus".to_string(), "Проблемы с RSA".to_string());
        } else { config.insert("UTMstatus".to_string(), "".to_string()); }
//>

// <get лицензия фсрар
//        if resp.contains("Лицензия на вид деятельности действует") {
//           config.insert("UTMlicense".to_string(), "1".to_string());
//        } else { config.insert("UTMlicense".to_string(), "0".to_string()); }
        config.insert("UTMlicense".to_string(), "1".to_string());
//>

// <get BuildNumber
        while start < words.len() {
            if words[start].contains("BuildNumber") {
                // item = start;
                break;
            }
            start += 1;
        }
        let mut tmpstr = String::new();
        let mut flag = false;
        for c in words[start + 4].chars() {
            if c == '>' {
                flag = true;
                continue;
            }
            if c == '<' { break }
            if flag { tmpstr.push(c); }
        }
        config.insert("utmversion".to_string(), tmpstr);
//>


// <get Неотправленные чеки
        while start < words.len() {
            if words[start].contains("Неотправленные") {
                // item = start;
                break;
            }
            start += 1;
        }
        if words[start + 5].contains("Отсутствуют") {
            tmpstr = String::from("Отсутствуют неотправленные чеки");
        } else if words[start + 5].contains("Чеки") {
            tmpstr = format!("Чеки {} {} {} {} {}", words[start + 6], words[start + 7], words[start + 8], words[start + 9], words[start + 10])
        } else { tmpstr = "".to_string() };
        config.insert("unsended_receipts".to_string(), tmpstr);
//>

// <RSA start, RSA end
        while start < words.len() {
            if words[start].contains("RSA") {
//                item = start;
                break;
            }
            start += 1;
        }
        config.insert("utmrsa_in".to_string(), words[start + 6].to_string());
        config.insert("utmrsa".to_string(), words[start + 10].to_string());
//>

// <GOST start, GOST end
        while start < words.len() {
            if words[start].contains("ГОСТ") {
                // item = start;
                break;
            }
            start += 1;
        }
        config.insert("utmgost_in".to_string(), words[start + 6].to_string());
        config.insert("utmgost".to_string(), words[start + 10].to_string());
//>

// <FSRAR-ID
        while start < words.len() {
            if words[start].contains("FSRAR-RSA-") {
                // item = start;
                break;
            }
            start += 1;
        }
        let tmpstr = &words[start][10..22];
        config.insert("fsrar".to_string(), tmpstr.to_string());
//>

//    println!("build - {:#?}",config);
    }

    Ok(config)
}