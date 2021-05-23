//use std::fs::File;
//use std::io::prelude::*;
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