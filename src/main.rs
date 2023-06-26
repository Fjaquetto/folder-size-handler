use std::fs;
use std::io::{self, Write};
use std::collections::HashMap;
use std::path::Path;

fn format_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if size < KB {
        return format!("{} B", size);
    } else if size < MB {
        return format!("{:.2} KB", (size as f64) / (KB as f64));
    } else if size < GB {
        return format!("{:.2} MB", (size as f64) / (MB as f64));
    } else {
        return format!("{:.2} GB", (size as f64) / (GB as f64));
    }
}

fn visit_dirs(dir: &Path, size_map: &mut HashMap<String, u64>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, size_map)?;
            } else {
                let metadata = fs::metadata(&path)?;
                let size = metadata.len();
                size_map.insert(String::from(path.to_str().unwrap()), size);
            }
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    loop {
        print!("Enter directory path: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let dir = input.trim_end();
        let path = Path::new(dir);

        let mut size_map: HashMap<String, u64> = HashMap::new();
        visit_dirs(&path, &mut size_map)?;

        let mut size_vec: Vec<(&String, &u64)> = size_map.iter().collect();
        size_vec.sort_by(|a, b| b.1.cmp(a.1));

        for (path, size) in size_vec {
            println!("{} ({})", path, format_size(*size));
        }

        println!("\n");
    }
}
