use crate::traits::command_trait;
use crate::commands::*;
use crate::structures::column::Column;
use crate::structures::database::Database;
use crate::structures::table::Table;

pub struct Bot {
    pub db: Database
}

impl Bot {
    pub async fn new() -> Self {
        Self {
            db: Database::default().await
        }
    }

    pub fn commands() -> Vec<Box<dyn command_trait::Command>> {
        vec![
            Box::new(info::ping::PingCommand)
        ]
    }

    pub fn tables() -> Vec<Table> {
        return vec![
            Table {
                name: "giveaways".to_string(),
                columns: vec![
                    Column {
                        name: "message_id".to_string(),
                        kind: "TEXT".to_string()
                    },
                    Column {
                        name: "title".to_string(),
                        kind: "TEXT".to_string()
                    },
                    Column {
                        name: "winners".to_string(),
                        kind: "INTEGER".to_string()
                    },
                    Column {
                        name: "user_id".to_string(),
                        kind: "TEXT".to_string()
                    },
                    Column {
                        name: "channel_id".to_string(),
                        kind: "TEXT".to_string()
                    },
                    Column {
                        name: "guild_id".to_string(),
                        kind: "TEXT".to_string()
                    },
                    Column {
                        name: "ends_at".to_string(),
                        kind: "INTEGER".to_string()
                    }
                ]
            }
        ]
    }
}

