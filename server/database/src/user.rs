use crate::driver::Database;
use crate::generate_string;
use crate::hash::{hash, verify, SALT_LENGTH};
use sqlx::{FromRow, Result};
use thiserror::Error;

#[derive(Debug, FromRow)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub is_admin: bool,
    password: Option<String>,
    salt: Option<String>,
    legacy_password: Option<String>,
}

#[derive(Debug, Error)]
pub enum UserError {
    #[error("{0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("{0}")]
    Bcrypt(#[from] bcrypt::BcryptError),
    #[error("No password is set")]
    NoPassword,
}

impl User {
    /// Create a new user in the database
    ///
    /// # Parameter notes
    /// - `password` should be the plaintext password.
    /// - `legacy_password` should be the password copied from constipated-koala's database.
    /// - `pepper` password pepper, only used if `password` is provided.
    pub async fn new(
        driver: &Database,
        name: String,
        email: String,
        is_admin: bool,
        password: Option<String>,
        legacy_password: Option<String>,
        pepper: &str,
    ) -> std::result::Result<Self, UserError> {
        let (salt, hash) = match password {
            Some(p) => {
                let salt = generate_string(SALT_LENGTH);
                let hash = hash(&p, &salt, &pepper)?;

                (Some(salt), Some(hash))
            }
            None => (None, None),
        };

        let id = generate_string(64);
        sqlx::query("INSERT INTO users (id, name, email, is_admin, password, salt, legacy_password) VALUES (?, ?, ?, ?, ?, ?, ?)")
            .bind(&id)
            .bind(&name)
            .bind(&email)
            .bind(is_admin)
            .bind(&hash)
            .bind(&salt)
            .bind(&legacy_password)
            .execute(&**driver)
            .await?;

        Ok(Self {
            name,
            email,
            id,
            is_admin,
            password: hash,
            salt,
            legacy_password,
        })
    }

    pub async fn get_by_id(driver: &Database, id: &str) -> Result<Option<Self>> {
        Ok(sqlx::query_as("SELECT * FROM users WHERE id = ?")
            .bind(id)
            .fetch_optional(&**driver)
            .await?)
    }

    pub async fn get_by_email(driver: &Database, email: &str) -> Result<Option<Self>> {
        Ok(sqlx::query_as("SELECT * FROM users WHERE email = ?")
            .bind(email)
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
            sqlx::query_scalar("SELECT scope FROM user_permitted_scopes WHERE user_id = ?")
                .bind(&self.id)
                .fetch_all(&**driver)
                .await?,
        )
    }

    pub async fn remove_permitted_scope(&self, driver: &Database, scope: &str) -> Result<()> {
        sqlx::query("DELETE FROM user_permitted_scopes WHERE user_id = ? AND scope = ?")
            .bind(&self.id)
            .bind(scope)
            .execute(&**driver)
            .await?;

        Ok(())
    }

    pub async fn grant_permitted_scope(&self, driver: &Database, scope: &str) -> Result<()> {
        sqlx::query("INSERT INTO user_permitted_scopes (user_id, scope) VALUES (?, ?)")
            .bind(&self.id)
            .bind(scope)
            .execute(&**driver)
            .await?;

        Ok(())
    }

    /// Update the password of the user.
    /// After this, [Self::clear_legacy_password] should be called.
    ///
    /// # Parameter notes
    /// - `password` should be provided in plain text.
    pub async fn set_password(
        &mut self,
        driver: &Database,
        password: &str,
        pepper: &str,
    ) -> std::result::Result<(), UserError> {
        let salt = generate_string(SALT_LENGTH);
        let hash = hash(password, &salt, pepper)?;

        sqlx::query("UPDATE users SET password = ?, salt = ? WHERE id = ?")
            .bind(&hash)
            .bind(&salt)
            .bind(&self.id)
            .execute(&**driver)
            .await?;

        self.salt = Some(salt);
        self.password = Some(hash);

        Ok(())
    }

    pub async fn check_password(
        &mut self,
        database: &Database,
        rhs: &str,
        pepper: &str,
    ) -> std::result::Result<bool, UserError> {
        match (&self.password, &self.legacy_password) {
            (Some(p), _) => Ok(verify(p, rhs, pepper)?),
            (None, Some(_p)) => {
                // Update the password to the new algorithm.
                self.set_password(database, rhs, pepper).await?;

                self.clear_legacy_password(database).await?;

                // Ok(true)

                panic!("Implement same hashing algo as constipated koala");
            }
            (None, None) => Err(UserError::NoPassword),
        }
    }

    pub async fn clear_legacy_password(
        &mut self,
        driver: &Database,
    ) -> std::result::Result<(), UserError> {
        if self.password.is_none() {
            return Err(UserError::NoPassword);
        }

        sqlx::query("UPDATE users SET legacy_password = NULL WHERE id = ?")
            .bind(&self.id)
            .execute(&**driver)
            .await?;

        self.legacy_password = None;

        Ok(())
    }
}
