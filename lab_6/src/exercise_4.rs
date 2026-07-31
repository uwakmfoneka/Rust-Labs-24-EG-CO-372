use std::fs::{self, File};
use std::io::{self, BufRead, Write};
use std::path::Path;

fn write_log(path: &str, entries: &[&str]) -> io::Result<()> {
    let mut file = File::create(path)?;

    for entry in entries {
        writeln!(file, "{}", entry)?;
    }

    Ok(())
}

fn count_lines(path: &str) -> io::Result<usize> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    Ok(reader.lines().count())
}

// TODO 4
fn list_rs_files(path: &Path) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            list_rs_files(&path)?;
        } else {
            if let Some(ext) = path.extension() {
                if ext == "rs" {
                    println!("{}", path.display());
                }
            }
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let path = "output.log";

    let entries = vec![
        "INFO Server started",
        "WARN High memory usage",
        "ERROR Disk full",
        "INFO Backup complete",
    ];

    write_log(path, &entries)?;

    let n = count_lines(path)?;
    println!("Wrote {} lines to {}", n, path);

    // Read back and filter
    let content = fs::read_to_string(path)?;

    let errors: Vec<&str> = content
        .lines()
        .filter(|l| l.starts_with("ERROR"))
        .collect();

    println!("Error lines: {:?}", errors);

    fs::remove_file(path)?;

    // TODO 4
    println!("Rust files:");
    list_rs_files(Path::new("."))?;

    Ok(())
}