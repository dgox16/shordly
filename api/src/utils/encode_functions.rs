use harsh::Harsh;

use crate::env_config::EnvConfig;

pub fn encode_id(id: i32) -> String {
    let env_config = EnvConfig::init();

    let harsh = Harsh::builder().salt(env_config.secret_id).build().unwrap();
    harsh.encode(&[id as u64])
}
