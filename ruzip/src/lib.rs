extern crate zip;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use zip::write::FileOptions;
use zip::{CompressionMethod, ZipArchive};

pub struct Config {
    pub option: String,
    pub file: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // program file name
        args.next();

        let option = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get an option"),
        };

        let file = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file"),
        };

        Ok(Config { option, file })
    }
}

fn compress(file: &str) -> Result<(), Box<dyn Error>> {
    let file_path = Path::new(file);
    let zip_path = file_path.with_extension("zip");

    let mut zip = zip::ZipWriter::new(File::create(&zip_path)?);
    let options = FileOptions::default().compression_method(CompressionMethod::Stored);

    zip.start_file(file_path.file_name().unwrap().to_str().unwrap(), options)?;
    let mut input = BufReader::new(File::open(file)?);
    std::io::copy(&mut input, &mut zip)?;

    zip.finish()?;
    Ok(())
}

fn decompress(file: &str) -> Result<(), Box<dyn Error>> {
    let file_path = Path::new(file);
    let zip_file = File::open(file_path)?;

    let mut archive = ZipArchive::new(zip_file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let file_name = file.mangled_name();

        let output_path = file_path.with_file_name(file_name);
        let mut output_file = File::create(&output_path)?;

        std::io::copy(&mut file, &mut output_file)?;
    }

    Ok(())
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let _exec = if config.option == "u" {
        decompress(&config.file)?
    } else {
        compress(&config.file)?
    };

    Ok(())
}
