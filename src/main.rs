use regex::Regex;
use std::env;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Error, Write};
use std::path::Path;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(Error::other("Filename missing."));
    }

    let filename_in = &args[1];
    let filename_out = filename_in.to_owned() + ".temp";

    {
        let file_in = File::open(Path::new(filename_in))?;
        let file_out = File::create(Path::new(&filename_out))?;

        // ^EXCLUDE_OBJECT_START NAME='     Start delimiter
        // (                    Optional
        //   (?<first>\d+)          First number part with at least one digit
        //   (                      Optional
        //     p|P                      'p' or 'P'
        //     (?<second>\d+)           Second number part with at least one digit
        //   )?
        // )?
        let Ok(re) =
            Regex::new(r"^EXCLUDE_OBJECT_START NAME='((?<first>\d+)(p|P(?<second>\d+))?)?")
        else {
            return Err(Error::other("Invalid regex."));
        };

        let reader = BufReader::new(file_in);
        let mut writer = BufWriter::new(file_out);

        for line in reader.lines() {
            let line = line?;

            writeln!(writer, "{}", line)?;

            let Some(captures) = re.captures(&line) else {
                continue;
            };

            let Some(first) = captures.name("first") else {
                writeln!(writer, "M221 S100")?;
                continue;
            };

            let second = match captures.name("second") {
                Some(second) => ".".to_owned() + second.as_str(),
                None => "".to_owned(),
            };

            writeln!(writer, "M221 S{}{}", first.as_str(), second)?;
        }
    }

    fs::rename(filename_out, filename_in)?;

    Ok(())
}
