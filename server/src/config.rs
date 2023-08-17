use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Env {
    #[serde(default)]
    pub random_disconnect: bool,
}

pub fn load() -> Env {
    dotenv::dotenv().ok();

    envy::from_env().expect("failed to load env")
}
