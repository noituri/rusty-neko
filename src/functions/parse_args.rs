use serenity::client::Context;
use serenity::model::channel::Message;
use crate::enums::arg_types::ArgTypes;
use crate::enums::raw_arg_types::RawArgTypes;
use crate::handlers::handle_arg_error::handle_arg_error;
use crate::structures::bot::Bot;
use crate::structures::extras::Extras;
use crate::traits::command_trait::Command;
use crate::util::parsers::parse_bool::parse_bool;
use crate::util::parsers::parse_int::parse_int;
use crate::util::parsers::parse_string::parse_string;

pub async fn parse_args(bot: &Bot, ctx: &Context, msg: &Message, command: &dyn Command, raw_args: Vec<&str>, extras: &Extras) -> Result<Vec<ArgTypes>, ()> {
    let mut parsed_args = Vec::<ArgTypes>::new();

    if command.args().is_empty() {
        raw_args.iter().for_each(| el | parsed_args.push(ArgTypes::String(el.to_string())));
        return Ok(parsed_args)
    }

    let args = command.args();
    let len = args.len();

    for i in 0..len {
        let arg = args.get(i).unwrap();
        let current = {
            if i + 1 == len {
                if raw_args.len() < i {
                    Err(())
                } else {
                    Ok(raw_args[i..].join(" "))
                }
            } else {
                let got = raw_args.get(i);
                if got.is_none() {
                    Err(())
                } else {
                    let d = got.unwrap().to_string();
                    if d.is_empty() {
                        return Err(())
                    }

                    Ok(d)
                }
            }
        };

        if current.is_err() {
            if arg.required {
                handle_arg_error(bot, ctx, command, extras, msg, arg, "".to_string(), "".to_string()).await;
                return Err(())
            }

            parsed_args.push(ArgTypes::Empty(()));
            continue;
        }

        let unzip = &*current.unwrap();

        match arg.expect {
            RawArgTypes::Bool => {
                let ps = parse_bool(arg, unzip);

                match ps {
                    Ok(ps) => {
                        parsed_args.push(ArgTypes::Bool(ps));
                        continue;
                    }
                    Err(ps) => {
                        handle_arg_error(bot, ctx, command, extras, msg, arg, unzip.to_string(), ps).await;
                        return Err(())
                    }
                }
            }

            RawArgTypes::Integer => {
                let ps = parse_int(arg, unzip).await;

                match ps {
                    Ok(ps) => {
                        parsed_args.push(ArgTypes::Int(ps));
                        continue;
                    }

                    Err(ps) => {
                        handle_arg_error(bot, ctx, command, extras, msg, arg, unzip.to_string(), ps).await;
                        return Err(())
                    }
                }
            },

            RawArgTypes::String => {
                let ps = parse_string(arg, unzip).await;

                match ps {
                    Ok(ps) => {
                        parsed_args.push(ArgTypes::String(ps));
                        continue;
                    }

                    Err(ps) => {
                        handle_arg_error(bot, ctx, command, extras, msg, arg, unzip.to_string(), ps).await;
                        return Err(())
                    }
                }
            }
        }
    }

    Ok(parsed_args)
}