use std::collections::HashMap;
use std::error::Error;

pub fn get(utm:&String) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut config: HashMap<String, String> = HashMap::new(); //итоговый хешмэп с конфигом
    let mut resp = ureq::get(utm).call()?.into_string()?;

// < error 403
    if resp.contains("HTTP ERROR 403") {
        config.insert("rsastatus".to_string(), "HTTP ERROR 403".to_string()); //UTMstatus
    } else {
//>
        resp.truncate(6701); // Отрезаем лишний конец
        let words: Vec<&str> = resp.split_whitespace().collect(); //весь ответ помещаем в вектор разделяя по пробелу
        let mut start = 100;

// <get проблемы с RSA
        if resp.contains("Проблемы с RSA") {
            config.insert("rsastatus".to_string(), "Проблемы с RSA".to_string());
        } else { config.insert("rsastatus".to_string(), "".to_string()); }
//>

// <get лицензия фсрар
//        if resp.contains("Лицензия на вид деятельности действует") {
//           config.insert("UTMlicense".to_string(), "1".to_string());
//        } else { config.insert("UTMlicense".to_string(), "0".to_string()); }
        config.insert("utmlicense".to_string(), "1".to_string());
//>

// <get utmversion
        while start < words.len() {
            if words[start].contains("Версия") {
                break;
            }
            start += 1;
        }
        let mut tmpstr = String::new();
        let mut flag = false;
        for c in words[start + 5].chars() {
            if c == '>' {
                flag = true;
                continue;
            }
            if c == '<' { break }
            if flag { tmpstr.push(c); }
        }
        config.insert("utmversion".to_string(), tmpstr);//
//>

// <get BuildNumber
        while start < words.len() {
            if words[start].contains("BuildNumber") {
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
        config.insert("buildNumber".to_string(), tmpstr);// utmversion
//>


// <get Неотправленные чеки
        while start < words.len() {
            if words[start].contains("Неотправленные") {
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
                break;
            }
            start += 1;
        }
        let tmpstr = &words[start][10..22];
        config.insert("fsrar".to_string(), tmpstr.to_string());
//>

    }

    Ok(config)
}