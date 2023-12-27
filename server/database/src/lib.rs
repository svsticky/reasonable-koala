use rand::Rng;

pub mod constant_access_tokens;
pub mod driver;
pub mod oauth2_client;
pub mod user;

fn generate_string(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(rand::distributions::Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}
