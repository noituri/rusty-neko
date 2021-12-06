use sqlx;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{Pool, Sqlite};
use crate::structures::bot::Bot;

pub struct Database {
    pub pool: sqlx::SqlitePool
}

impl Database {
    pub async fn default() -> Self {
        let pool: Pool<Sqlite> = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(
                SqliteConnectOptions::new()
                    .filename("./neko.db")
                    .create_if_missing(true)
            )
            .await
            .expect("Could not connect to database.");

        let tables = Bot::tables();

        for tbl in tables {
            let mut columns = tbl.columns;

            let primclm = columns.remove(0);

            sqlx::query(
                &format!(
                    "CREATE TABLE IF NOT EXISTS {}({} {})",
                    tbl.name.to_string(),
                    primclm.name.to_string(),
                    primclm.kind.to_string()
                )
            )
                .execute(&pool)
                .await
                .expect("Failed to create table");

            for clm in columns {
                let res = sqlx::query(
                    &format!(
                        "ALTER TABLE {} ADD COLUMN {} {}",
                        tbl.name.to_string(),
                        clm.name.to_string(),
                        clm.kind.to_string()
                    )
                )
                    .execute(&pool)
                    .await;

                match res {
                    Ok(_) => {
                        println!(
                            "{}",
                            format!(
                                "Successfully created column {} in table {}.",
                                clm.name.to_string(),
                                tbl.name.to_string()
                            )
                        )
                    },
                    Err(res) => {
                        println!(
                            "{}",
                            format!(
                                "Failed to create column {} at table {}: {:?}.",
                                clm.name.to_string(),
                                tbl.name.to_string(),
                                res
                            )
                        )
                    }
                }
            }
        }

        Self {
            pool
        }
    }
}
