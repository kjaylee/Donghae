// src/main.rs

mod network;
mod consensus;
mod poh;
mod vm;
mod storage;
mod utils;

use env_logger::Env;
use log::info;

#[tokio::main]
async fn main() {
    // 로그 설정을 초기화합니다.
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    info!("Donghae 블록체인 노드를 시작합니다.");

    // 스토리지 모듈을 초기화합니다.
    let storage = storage::Storage::new();

    // 네트워크 모듈을 초기화하고, 비동기적으로 실행합니다.
    tokio::spawn(async {
        network::start_network().await;
    });

    // 합의 알고리즘을 시작하고, 비동기적으로 실행합니다.
    tokio::spawn(async {
        consensus::start_consensus().await;
    });

    // 메인 스레드에서 추가 작업을 수행하거나 대기합니다.
    loop {
        // 필요에 따라 추가 작업을 수행합니다.
        // 예: 사용자 입력 처리, 상태 모니터링 등

        // 1초 대기 후 루프를 계속합니다.
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}
