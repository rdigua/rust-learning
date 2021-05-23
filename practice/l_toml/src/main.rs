///
/// Cargo clippy --all.
/// Wed May 12 11:23:50     2021
/// 

use l_toml::{read1, reader};



fn main() {
    let s: String = read1("re.toml");

    let config: l_toml::Config = match toml::from_str(&s) {
        Ok(str1) => str1,
        _ => {
            println!("File format style had some error!");
            return;
        }
    };

    //let config:Config=toml::from_str(&s).unwrap();
    println!("{:?}", config);

    let s1: String = match reader("re.toml") {
        Some(s) => s,
        _ => {
            println!("Error Reading file");
            return;
        }
    };

    let config: l_toml::Config = match toml::from_str(&s1) {
        Ok(str1) => str1,
        _ => {
            println!("File format style had some error!");
            return;
        }
    };

    //let config:Config=toml::from_str(&s).unwrap();
    println!("{:?}", config);
}
