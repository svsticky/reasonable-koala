use rand::Rng;

pub mod driver;
pub mod oauth2_client;
pub mod access_token;

fn generate_string(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(rand::distributions::Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}