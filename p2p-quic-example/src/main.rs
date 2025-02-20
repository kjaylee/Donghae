use libp2p::{
    core::transport::upgrade,
    identity,
    libp2p_quic as quic,
    libp2p::SwarmBuilder,
    libp2p_core::{Multiaddr, Transport, transport::ListenerId},
    PeerId, Transport,
};
use async_std::task;
use futures::stream::StreamExt;

fn main() {
    task::block_on(async {
        // 피어 ID 및 키 생성
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());
        println!("Local peer id: {:?}", local_peer_id);

        // QUIC 트랜스포트 설정
        let transport = QuicTransport::new(QuicConfig::default())
            .upgrade(upgrade::Version::V1)
            .boxed();

        // Swarm 생성
        let mut swarm = SwarmBuilder::new(transport, (), local_peer_id)
            .executor(Box::new(|fut| {
                async_std::task::spawn(fut);
            }))
            .build();

        // 이벤트 루프
        loop {
            match swarm.next().await {
                Some(event) => println!("Event: {:?}", event),
                None => break,
            }
        }
    });
}
