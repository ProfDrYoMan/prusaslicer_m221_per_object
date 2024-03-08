use regex::Regex;
use std::env;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Error, Write};
use std::path::Path;

fn main() -> Result<(), Error> {
    // Check for command line parameter
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(Error::other("Filename missing."));
    }

    // Build filenames
    let filename_in = &args[1];
    let filename_out = filename_in.to_owned() + ".temp";

    // Try to get original extrusion multiplier
    let original_multiplier = get_original_multiplier(filename_in);

    // Process g-code file
    {
        let file_in = File::open(Path::new(filename_in))?;
        let file_out = File::create(Path::new(&filename_out))?;

        // ^EXCLUDE_OBJECT_START NAME='     Start delimiter
        // (                    Optional
        //   (?<first>\d+)          First number part with at least one digit
        //   (                      Optional
        //     (
        //       (p|P)                  ('p' or 'P')
        //       |                      or
        //       (?<modi>m|M)           ('m' or 'M')
        //     )
        //     (?<second>\d+)           Second number part with at least one digit
        //   )?
        // )?
        let Ok(re) = Regex::new(
            r"^EXCLUDE_OBJECT_START NAME='((?<first>\d+)(((p|P)|(?<modi>m|M))(?<second>\d+))?)?",
        ) else {
            return Err(Error::other("Invalid regex."));
        };

        let reader = BufReader::new(file_in);
        let mut writer = BufWriter::new(file_out);

        // Output info
        writeln!(
            writer,
            concat!(
                "; Post-processed by prusaslicer_m221_per_object\n",
                "; https://github.com/ProfDrYoMan/prusaslicer_m221_per_object\n",
                "; Original extrusion multiplier: {}\n"
            ),
            original_multiplier
        )?;

        // Process lines of g-code file
        for line in reader.lines() {
            let line = line?;

            // Write every line to output file
            writeln!(writer, "{}", line)?;

            let Some(captures) = re.captures(&line) else {
                // Next line if regex does not match
                continue;
            };

            let Some(first) = captures.name("first") else {
                // If no matching object name is found, use default flow rate
                writeln!(
                    writer,
                    concat!(
                        "; prusaslicer_m221_per_object: Keep default flow rate\n",
                        "M221 S100"
                    )
                )?;
                continue;
            };

            let second = match captures.name("second") {
                // Use the match after 'p' or 'P' or 'm' or 'M'
                Some(second) => second.as_str(),
                // Or use zero
                None => "0",
            };

            // Build a number of first and second part or use 100% as sanity
            // (should never happen)
            let number = format!("{}.{}", first.as_str(), second);
            let mut number: f64 = number.parse().unwrap_or(1.0);

            match captures.name("modi") {
                // On 'm' or 'M' as decimal point calculate a new extrusion multiplier
                // based on the original extrusion multiplier
                Some(_modifier) => {
                    number = (1000.0 * 100.0 * number / original_multiplier).round() / 1000.0;
                    writeln!(
                        writer,
                        "; prusaslicer_m221_per_object: Set flow rate from extrusion multiplier",
                    )?;
                }
                // On 'p' or 'P' as decimal point use the explicit extrusion multiplier
                None => {
                    writeln!(
                        writer,
                        "; prusaslicer_m221_per_object: Set flow rate from explicit setting",
                    )?;
                }
            };

            writeln!(writer, "M221 S{}", number)?;
        }
    }

    // Move temporary file over the original g-code file
    fs::rename(filename_out, filename_in)?;

    return Ok(());
}

// Get the original multiplier, either from environment or the g-code
fn get_original_multiplier(filename: &str) -> f64 {
    let Some(multiplier) = get_original_multiplier_from_environment() else {
        eprintln!(concat!(
            "Warning:\n",
            " SLIC3R_EXTRUSION_MULTIPLIER environment variable not set.\n",
            " Parsing g-code for multiplier."
        ));

        // Not in environment, try g_code
        let Some(multiplier) = get_original_multiplier_from_g_code(filename) else {
            eprintln!(concat!(
                "Warning:\n",
                " No extrusion multiplier found.\n",
                " Assuming an extrusion multiplier of 100%."
            ));

            return 1.0;
        };

        return multiplier;
    };

    return multiplier;
}

// Get the original multiplier from the environment variable SLIC3R_EXTRUSION_MULTIPLIER
fn get_original_multiplier_from_environment() -> Option<f64> {
    let Ok(multiplier) = env::var("SLIC3R_EXTRUSION_MULTIPLIER") else {
        return None;
    };

    return multiplier.parse().ok();
}

// Get the original multiplier from the g-code
fn get_original_multiplier_from_g_code(filename: &str) -> Option<f64> {
    let Ok(file) = File::open(Path::new(filename)) else {
        return None;
    };

    let reader = BufReader::new(file);

    let re = Regex::new(r"^; extrusion_multiplier = (?<multi>.+)$").ok()?;

    for line in reader.lines() {
        let line = line.ok()?;

        let Some(captures) = re.captures(&line) else {
            continue;
        };

        let Some(multiplier) = captures.name("multi") else {
            return None;
        };

        return multiplier.as_str().parse().ok();
    }

    return None;
}
