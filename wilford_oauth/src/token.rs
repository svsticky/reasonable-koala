use reqwest::{Client, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct Tokens {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub refresh_token: String,
    pub scope: String,
}

#[derive(Serialize)]
struct Query<'a> {
    grant_type: GrantType,
    code: Option<&'a str>,
    redirect_uri: &'a str,
    client_id: &'a str,
    refresh_token: Option<&'a str>,
    client_secret: &'a str,
}

#[derive(Serialize)]
pub enum GrantType {
    #[serde(rename(deserialize = "authorization_code"))]
    AuthorizationCode,
    #[serde(rename(deserialize = "refresh_token"))]
    RefreshToken,
}

impl Tokens {
    fn token_endpoint(wilford: &str) -> String {
        format!("{wilford}/api/oauth/token")
    }

    pub fn authorization_redirect(
        wilford: &str,
        client_id: &str,
        redirect_uri: &str,
        scope: Option<&str>,
        state: Option<&str>,
    ) -> String {
        #[derive(Serialize)]
        struct Query<'a> {
            response_type: &'static str,
            client_id: &'a str,
            redirect_uri: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            scope: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            state: Option<&'a str>,
        }

        let query = serde_qs::to_string(&Query {
            response_type: "code",
            client_id,
            redirect_uri,
            scope,
            state,
        })
        .expect("Serializing query");

        format!("{wilford}/api/oauth/authorize?{query}")
    }

    pub async fn exchange_code(
        wilford: &str,
        code: &str,
        client_id: &str,
        client_secret: &str,
        redirect_uri: &str,
    ) -> Result<Self> {
        Ok(Client::new()
            .post(Self::token_endpoint(wilford))
            .query(&Query {
                grant_type: GrantType::AuthorizationCode,
                code: Some(code),
                client_id,
                client_secret,
                refresh_token: None,
                redirect_uri,
            })
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }

    pub async fn refresh_token(
        wilford: &str,
        refresh_token: &str,
        client_id: &str,
        client_secret: &str,
        redirect_uri: &str,
    ) -> Result<Self> {
        Ok(Client::new()
            .post(Self::token_endpoint(wilford))
            .query(&Query {
                client_id,
                redirect_uri,
                client_secret,
                refresh_token: Some(refresh_token),
                code: None,
                grant_type: GrantType::RefreshToken,
            })
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }
}
