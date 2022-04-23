use crate::{
    models::{ConfigInsert, ConfigQuery},
    schema::config::dsl::{config, key, value},
};
use diesel::query_dsl::RunQueryDsl;
use diesel::ExpressionMethods;
use diesel::{OptionalExtension, QueryDsl, SqliteConnection};
use serenity::framework::standard::{CommandError, CommandResult};

pub fn get(k: &str, conn: &mut SqliteConnection) -> Result<Option<String>, CommandError> {
    let config_query: Option<ConfigQuery> = config
        .filter(key.eq(k))
        .first::<ConfigQuery>(conn)
        .optional()?;
    if let Some(config_query) = config_query {
        Ok(Some(config_query.value))
    } else {
        Ok(None)
    }
}

pub fn set(k: &str, v: &str, conn: &mut SqliteConnection) -> CommandResult {
    diesel::insert_into(config)
        .values(ConfigInsert { key: k, value: v })
        .on_conflict(key)
        .do_update()
        .set(value.eq(v))
        .execute(conn)?;
    Ok(())
}
