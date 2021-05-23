use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use serde_derive::Deserialize;


#[derive(Deserialize, Debug)]
pub struct Config {
pub    title: Option<String>,
pub    bibizyz: Bibizyz,
}

#[derive(Deserialize, Debug)]
pub struct Bibizyz {
    pub    website: String,
    pub    baseurl: Option<String>,
    pub    listreg: Option<String>,
    pub    elementreg: Option<String>,
    pub    showreg: Option<String>,
}

///
/// Discard.
/// Practice.
/// 

pub fn read1<T: AsRef<Path>>(path: T) -> String {
    let mut file = match File::open(path.as_ref()) {
        Ok(f) => f,
        Err(e) => panic!("no such file {:?} exception:{}", path.as_ref(), e),
    };
    let mut str_val = String::new();
    match file.read_to_string(&mut str_val) {
        Ok(s) => s,
        Err(e) => panic!("Error Reading file: {}", e),
    };
    str_val
}

///
/// To get String From file .
/// When Err return None.
/// 

pub fn reader<T: AsRef<Path>>(path: T) -> Option<String> {
    let str_val: String = match std::fs::read_to_string(path.as_ref()) {
        Ok(s) => s,
        Err(e) => {
            println!("Error Reading file: {}", e);
            return None;
        }
    };

    Some(str_val)
}
