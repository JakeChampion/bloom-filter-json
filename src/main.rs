use serde_json;
use growable_bloom_filter::GrowableBloom;
use std::fs::File;
use std::io::Write;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./src/names.txt") {
        // Create and insert into the bloom filter
        let mut gbloom = GrowableBloom::new(0.01, 500_000);

        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(name) = line {
                gbloom.insert(name);
            }
        }
        let s = serde_json::to_string(&gbloom).unwrap();
        let path = "names.json";
        let mut output = File::create(path).unwrap();
        write!(output, "{}", s).unwrap()
    }
}
