use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, BufRead, Result, Write};
use std::process::{Command, Stdio};
use super::*;


/* Return a read-only handle to the `input-event-codes.h` header file.
 * The path is resolved by invoking the C preprocessor. */
pub fn get_input_codes_header() -> Result<File> {
    const INCLUDE_INPUT_CODES_HEADER: &[u8] =
        b"#include <linux/input-event-codes.h>";

    let cpp = Command::new("cpp")
        .arg("-H").arg("-o").arg("/dev/null")
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn();

    match cpp {
        Ok(mut cpp) => {
            if let Some(mut stdin) = cpp.stdin.take() {
                stdin.write(INCLUDE_INPUT_CODES_HEADER)?;
            }

            let output = String::from_utf8_lossy(&cpp.wait_with_output()?.stderr)
                .into_owned();

            let path = output.split_whitespace()
                .nth(1)
                .unwrap_or_default();

            File::options().read(true).open(path)
        },
        Err(_) => File::options()
            .read(true)
            .open("/usr/include/linux/input-event-codes.h")
    }
}


/* Return a mapping from the given event code names to their values. 
 * If no value for a given event code was found, then it will have no mapping
 * in the returned value. */
pub fn lookup<'a>(codes: &HashSet<&'a str>, event_codes_header: File)
    -> Result<HashMap<&'a str, c_ushort>>
{
    const C_DEFINE: &str = "#define";
    const HEX_PREFIX: &str = "0x";

    let mut values = HashMap::new();

    let input_codes_header = BufReader::new(event_codes_header);
    let lines = input_codes_header
        .lines()
        .filter_map(|l| l.ok())
        .filter(|l| l.starts_with(C_DEFINE));

    for line in lines {
        let mut s = line.split_whitespace().skip(1).take(2);
        let code = s.next();
        let value = s.next().map(|v| {
            if v.starts_with(HEX_PREFIX) {
                // Parse hexadecimal value
                let v = v.trim_start_matches(HEX_PREFIX);
                c_ushort::from_str_radix(v, 16).ok()
            } else {
                // Parse decimal value
                v.parse::<c_ushort>().ok()
            }
        }).flatten();

        if let Some((code, value)) = code.zip(value) {
            if let Some(code) = codes.get(code) {
                values.insert(*code, value);
            }
        }

        if codes.len() == values.len() {
            // Stop iterating early if every event code has been parsed
            break;
        }
    }

    Ok(values)
}
