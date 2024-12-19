use lazy_static::lazy_static;
use std::env;
lazy_static! {
    pub static ref WSS_URL: String = env::var("DEV_WSS_URL").expect("WSS_URL must be set");
    pub static ref VRF_CONTRACT_ADDRESS_ETH: String = env::var("VRF_ADDRESS").expect("VRF_ADDRESS must be set");
    pub static ref SIGNER_PRIVATE_KEY: String = env::var("SIGNER_PRIVATE_KEY").expect("SIGNER_PRIVATE_KEY must be set");
}

