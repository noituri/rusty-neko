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
            // AFAIK sqlx::query!() macro is safer because it does compile-time checks and escapes the passed values ~ noituri
            sqlx::query(
                &format!(
                    "CREATE TABLE IF NOT EXISTS {}({} {})",
                    tbl.name,
                    primclm.name,
                    primclm.kind
                )
            )
                .execute(&pool)
                .await
                .expect("Failed to create table");

            for clm in columns {
                let res = sqlx::query(
                    &format!(
                        "ALTER TABLE {} ADD COLUMN {} {}",
                        tbl.name,
                        clm.name,
                        clm.kind
                    )
                )
                    .execute(&pool)
                    .await;

                match res {
                    Ok(_) => {
                        println!(
                            "Successfully created column {} in table {}.",
                            clm.name,
                            tbl.name
                        )
                    },
                    Err(res) => {
                        println!(
                            "Failed to create column {} at table {}: {:?}.",
                            clm.name,
                            tbl.name,
                            res
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
