use crate::structures::arg::Arg;
use crate::structures::extras::Extras;
use crate::traits::command_trait::Command;

pub fn get_command_usage(command: &Box<dyn Command>, extras: &Extras) -> Vec<String> {
    let mut vc = Vec::<String>::new();

    let args = &command.args();
    let len = args.len();

    let howmany = args.iter().filter(| m | !m.required).collect::<Vec<&Arg>>().len();

    for i in 0..howmany + 1 {
        let mut strv = vec![
            format!(
                "{}{}",
                extras.prefix,
                extras.command_string
            )
        ];

        let fields: Vec<&Arg> = args[0..len - howmany + i].iter().collect();
        for fld in fields {
            if fld.required {
                strv.push(
                    format!(
                        "<{}>",
                        fld.name.to_owned()
                    )
                )
            } else {
                strv.push(
                    format!(
                        "[{}]",
                        fld.name.to_owned()
                    )
                )
            }
        }
        vc.push(strv.join(" "));
    }

    vc
}