use crate::structures::arg::Arg;

/**
 * Parses a string and validates it, else returns the error.
 */
pub async fn parse_string(arg: &Arg, input: &str) -> Result<String, String> {
    for reg in arg.regexes.iter() {
        if !reg.is_match(input) {
            return Err(
                format!(
                    "Supplied string does not match given regexp: `{}`",
                    reg
                )
            );
        }
    }

    if arg.min_len != 0 {
        if input.len() < arg.min_len {
            return Err(
                format!(
                    "Supplied string does not match minimum argument length: `{}`",
                    arg.min_len
                )
            );
        }
    }

    if arg.max_len != 0 {
        if input.len() > arg.max_len {
            return Err(
                format!(
                    "Supplied string does not match maximum argument length: `{}`",
                    arg.max_len
                )
            )
        }
    }

    Ok(input.to_owned())
}