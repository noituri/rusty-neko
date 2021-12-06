use crate::structures::extras::Extras;
use crate::traits::command_trait::Command;

pub fn get_command_usage(command: &Box<dyn Command>, extras: &Extras) -> Vec<String> {
    let mut vc = Vec::<String>::new();

    let args = &command.args();
    let len = args.len();

    let mut y: usize = 0;

    for _ in 0..len {
        let mut strv = vec![
            format!(
                "{}{}",
                extras.prefix,
                extras.command_string
            )
        ];

        let mut total: usize = 0;

        for x in 0..len {
            let arg = args.get(x).unwrap();

            if !arg.required {
                if y != 0 || x == 0 {
                    strv.push(
                        format!(
                            "[{}]",
                            arg.name.to_string()
                        )
                    )
                };
            } else {
                strv.push(
                    format!(
                        "<{}>",
                        arg.name.to_string()
                    )
                )
            };

            if !arg.required {
                if y == 0 {
                    y += 1;
                    break;
                } else if y == total {
                    y += 1;
                    break;
                }
            }

            total += 1;
        }

        vc.push(strv.join(" "));
    }

    vc
}