use glob::glob;
use itertools::Itertools;
use regex::Regex;

use std::fs;
use std::io;

fn main() {
    let mut args = std::env::args();
    let cmd = args.next().expect("Unexpected");
    if let Some((root, id)) = args.collect_tuple() {
        match process(root, id) {
            Ok(()) => {}
            Err(err) => println!("{err}"),
        }
    } else {
        println!("Usage:\n\t {cmd} <root> <id>");
    }
}

fn process(root: String, book: String) -> anyhow::Result<()> {
    let id = book.parse::<u32>()?;
    let rx = Regex::new("fb2-([0-9]+)-([0-9]+)")?;
    let mask = format!("{root}/fb2-*.zip");

    for entry in glob(&mask)? {
        let path = entry?;
        if let Some(name) = path.file_name() {
            let name = name.to_string_lossy();
            if let Some(caps) = rx.captures(&name) {
                let min = caps.get(1).map_or("", |m| m.as_str()).parse::<u32>()?;
                let max = caps.get(2).map_or("", |m| m.as_str()).parse::<u32>()?;
                println!("{:?} -> {name} -> {:?} {:?}", path.display(), min, max);
                if min <= id && id <= max {
                    let needle = format!("{book}.fb2");
                    let file = fs::File::open(&path)?;
                    let mut archive = zip::ZipArchive::new(file)?;
                    
                    if let Ok(mut file) = archive.by_name(&needle) {
                        println!("Filename: {} found", needle);
                        let mut outfile = fs::File::create(&needle)?;
                        io::copy(&mut file, &mut outfile)?;
                        return Ok(()); 
                    };
                }
            }
        }
    }
    Ok(())
}
