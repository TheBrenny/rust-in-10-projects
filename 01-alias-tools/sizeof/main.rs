#![allow(non_snake_case)]
use std::path::Path;

fn getSize(t: &&Path) -> (String, u64) {
    let meta = t.metadata().unwrap();
    let canonPath = t.canonicalize().unwrap().display().to_string();

    let mut totalSize: u64 = meta.len();

    if t.is_symlink() {
        totalSize = t.read_link().unwrap().metadata().unwrap().len();
    } else if t.is_dir() {
        totalSize = 0;
        for f in t.read_dir().expect("cannot read dir") {
            // Doing this is so much easier than f.unwrap!
            let f = f.expect("unable to get dir entry");
            let p = f.path();
            let (_, size) = getSize(&p.as_path());
            totalSize += size;
        }
    }

    return (canonPath, totalSize);
}

fn normaliseSize(size: u64) -> String {
    let bytes = size % 1024;
    let kilobytes: f64 = ((size as f64) / 1024.0) % 1024.0;
    let megabytes: f64 = ((size as f64) / 1024.0 / 1024.0) % 1024.0;
    let gigabytes: f64 = ((size as f64) / 1024.0 / 1024.0 / 1024.0) % 1024.0;
    let terabytes: f64 = ((size as f64) / 1024.0 / 1024.0 / 1024.0 / 1024.0) % 1024.0;
    let petabytes: f64 = ((size as f64) / 1024.0 / 1024.0 / 1024.0 / 1024.0 / 1024.0) % 1024.0;

    return if petabytes > 1.0 {
        format!("{:.2}", petabytes) + " pb"
    } else if terabytes > 1.0 {
        format!("{}", terabytes) + " tb"
    } else if gigabytes > 1.0 {
        format!("{:.2}", gigabytes) + " gb"
    } else if megabytes > 1.0 {
        format!("{:.2}", megabytes) + " mb"
    } else if kilobytes > 1.0 {
        format!("{:.2}", kilobytes) + " kb"
    } else {
        format!("{}", bytes) + " b"
    };
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let targets: Vec<String> = if args.len() > 1 {
        args[1..].to_vec()
    } else {
        vec![".".to_string()]
    };

    // Resolve the targets to absolute paths
    let targets = targets.iter().map(|t| Path::new(t)).collect::<Vec<&Path>>();

    let sizes = targets
        .iter()
        .map(getSize)
        .map(|(name, size)| (name, normaliseSize(size)))
        .collect::<Vec<_>>();

    for (name, size) in sizes {
        println!("{}\t--  {}", name, size);
    }
}
