use std::convert::TryInto;
use serenity::client::Context;
use serenity::model::channel::Message;
use crate::enums::arg_types::arg_types;
use crate::enums::raw_arg_types::raw_arg_types;
use crate::structures::bot::Bot;
use crate::traits::command_trait::Command;
use crate::util::parsers::parse_int::parse_int;
use crate::util::parsers::parse_string::parse_string;

pub async fn parse_args(bot: &Bot, ctx: &Context, msg: &Message, command: &Box<dyn Command>, raw_args: Vec<&str>) -> Result<Vec<arg_types>, ()> {
    let mut parsed_args = Vec::<arg_types>::new();

    if command.args().len() == 0 {
        return Ok(parsed_args)
    }

    let args = command.args();
    let len = args.len();

    for i in 0..len {
        let arg = args.get(i).unwrap();
        let current = {
            if i + 1 == len {
                Ok(raw_args[i..].join(" "))
            } else {
                let got = raw_args.get(i);
                if got.is_none() {
                    Err(())
                } else {
                    Ok(got.unwrap().to_string())
                }
            }
        };

        if current.is_err() {
            if arg.required {
                return Err(())
            }

            parsed_args.push(arg_types::Empty(()));
            continue;
        }

        let unzip = &*current.unwrap();

        match arg.expect {
            raw_arg_types::Integer => {
                let ps = parse_int(arg, unzip).await;

                match ps {
                    Ok(ps) => {
                        parsed_args.push(arg_types::Int(ps));
                        continue;
                    }

                    Err(ps) => {
                        return Err(())
                    }
                }
            },

            raw_arg_types::String => {
                let ps = parse_string(arg, unzip).await;

                match ps {
                    Ok(ps) => {
                        parsed_args.push(arg_types::String(ps));
                        continue;
                    }

                    Err(ps) => {
                        return Err(())
                    }
                }
            }
        }
    }

    Ok(parsed_args)
}