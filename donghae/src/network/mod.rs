// src/network/mod.rs

pub mod p2p;
pub mod quic;

use log::info;

pub async fn start_network() {
    info!("네트워크 모듈을 초기화합니다.");

    // P2P 네트워크를 시작합니다.
    p2p::start_p2p_network().await;

    // QUIC 프로토콜을 설정하고 리스너를 시작합니다.
    quic::start_quic_listener().await;
}
