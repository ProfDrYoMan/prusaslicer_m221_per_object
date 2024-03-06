use regex::Regex;
use std::env;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Missing file name.")
    }

    let filename_in = &args[1];
    let filename_out = filename_in.to_owned() + ".temp";

    {
        // let path_in = Path::new(filename_in);
        let file_in = match File::open(Path::new(filename_in)) {
            Err(why) => panic!("Couldn't open {}: {}", filename_in, why),
            Ok(file_in) => file_in,
        };

        let file_out = match File::create(Path::new(&filename_out)) {
            Err(why) => panic!("Couldn't create {}: {}", filename_out, why),
            Ok(file_out) => file_out,
        };

        let re = Regex::new(r"^EXCLUDE_OBJECT_START NAME='((?<first>\d+)(p|P(?<second>\d+))?)?")
            .unwrap();

        let reader = BufReader::new(file_in);
        let mut writer = BufWriter::new(file_out);

        for line in reader.lines() {
            let line = line.unwrap();

            writeln!(writer, "{}", line).unwrap();

            let Some(captures) = re.captures(&line) else {
                continue;
            };

            let Some(first) = captures.name("first") else {
                writeln!(writer, "M221 S100").unwrap();
                continue;
            };

            let first = first.as_str();

            let second = match captures.name("second") {
                Some(second) => ".".to_owned() + second.as_str(),
                None => "".to_owned(),
            };

            writeln!(writer, "M221 S{}{}", first, second).unwrap();
        }
    }

    fs::rename(filename_out, filename_in).unwrap();
}
