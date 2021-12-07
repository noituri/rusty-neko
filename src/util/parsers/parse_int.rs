use crate::structures::arg::Arg;

pub async fn parse_int(arg: &Arg, input: &str) -> Result<i64, String> {
    let cast = input.parse::<i64>();

    match cast {
        Ok(cast) => {
            let i: i64 = cast;

            if arg.min_len != 0 {
                if i < 0 || arg.min_len > (i as usize) {
                    return Err(
                        format!(
                            "Supplied int does not fit in the minimum argument size: `{}`",
                            arg.min_len
                        )
                    )
                } 
            }

            if arg.max_len != 0 {
                if arg.max_len < (i as usize) {
                    return Err(
                        format!(
                            "Supplied int does not fit in the maximum argument size: `{}`",
                            arg.max_len
                        )
                    )
                } 
            }

            Ok(i)
        },
        Err(cast) => {
            let d: std::num::ParseIntError = cast;
            return Err(
                format!(
                    "Failed to parse `{}` to int: {}",
                    input,
                    d 
                )
            );
        }
    }
}