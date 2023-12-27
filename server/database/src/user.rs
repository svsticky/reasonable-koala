use crate::driver::Database;
use sqlx::{FromRow, Result};

#[derive(Debug, FromRow)]
pub struct User {
    pub espo_user_id: String,
    pub name: String,
    pub is_espo_admin: bool,
}

impl User {
    pub async fn new(
        driver: &Database,
        espo_user_id: String,
        name: String,
        is_espo_admin: bool,
    ) -> Result<Self> {
        sqlx::query("INSERT INTO users (espo_user_id, name, is_espo_admin) VALUES (?, ?, ?)")
            .bind(&espo_user_id)
            .bind(&name)
            .bind(is_espo_admin)
            .execute(&**driver)
            .await?;

        Ok(Self {
            name,
            espo_user_id,
            is_espo_admin,
        })
    }

    pub async fn get_by_id(driver: &Database, id: &str) -> Result<Option<Self>> {
        Ok(sqlx::query_as("SELECT * FROM users WHERE espo_user_id = ?")
            .bind(id)
            .fetch_optional(&**driver)
            .await?)
    }

    pub async fn list(driver: &Database) -> Result<Vec<Self>> {
        Ok(sqlx::query_as("SELECT * FROM users")
            .fetch_all(&**driver)
            .await?)
    }

    pub async fn list_permitted_scopes(&self, driver: &Database) -> Result<Vec<String>> {
        Ok(
            sqlx::query_scalar("SELECT scope FROM user_permitted_scopes WHERE espo_user_id = ?")
                .bind(&self.espo_user_id)
                .fetch_all(&**driver)
                .await?,
        )
    }

    pub async fn remove_permitted_scope(&self, driver: &Database, scope: &str) -> Result<()> {
        sqlx::query("DELETE FROM user_permitted_scopes WHERE espo_user_id = ? AND scope = ?")
            .bind(&self.espo_user_id)
            .bind(scope)
            .execute(&**driver)
            .await?;

        Ok(())
    }

    pub async fn grant_permitted_scope(&self, driver: &Database, scope: &str) -> Result<()> {
        sqlx::query("INSERT INTO user_permitted_scopes (espo_user_id, scope) VALUES (?, ?)")
            .bind(&self.espo_user_id)
            .bind(scope)
            .execute(&**driver)
            .await?;

        Ok(())
    }
}
