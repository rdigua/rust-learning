//Compress /var/log directory into archive.tar.gz.
//Creates a File wrapped in GzEncoder and tar::Builder.
//Adds contents of /var/log directory recursively into the archive under backup/logspath with Builder::append_dir_all. GzEncoder is responsible for transparently compressing the data prior to writing it into archive.tar.gz.
extern crate tar;
extern crate flate2;

use std::fs::File;
use flate2::Compression;
use flate2::write::GzEncoder;

fn main() -> Result<(), std::io::Error> {
    let tar_gz = File::create("archive.tar.gz")?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all("backup/logs", "/var/log")?;
    Ok(())
}

