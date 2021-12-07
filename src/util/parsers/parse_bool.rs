use crate::structures::arg::Arg;

pub const VALID_TRUE_VALUES: [&str; 3] = ["true", "yes", "on"];

pub const VALID_FALSE_VALUES: [&str; 3] = ["false", "no", "off"];

pub fn parse_bool(_arg: &Arg, input: &str) -> Result<bool, String> {
    for i in VALID_TRUE_VALUES {
        if i.eq(input) {
            return Ok(true);
        }
    }

    for i in VALID_FALSE_VALUES {
        if i.eq(input) {
            return Ok(false);
        }
    }

    Err(format!("Failed to parse `{}` into a boolean.", input))
}
