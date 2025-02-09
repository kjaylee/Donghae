// src/network/p2p.rs

use libp2p::{
    core::upgrade,
    dns::TokioDnsConfig,
    identify::{Behaviour as IdentifyBehaviour, Config as IdentifyConfig, Event as IdentifyEvent},
    identity,
    noise,
    swarm::{NetworkBehaviour, Swarm, SwarmEvent},
    tcp::tokio::Transport as TokioTcpTransport,
    yamux,
    Transport,
    PeerId,
};
use futures::StreamExt;
use log::info;
use tokio::io;

pub async fn start_p2p_network() {
    info!("P2P 네트워크를 시작합니다.");

    // 로컬 키 쌍 생성
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    info!("로컬 Peer ID: {:?}", local_peer_id);

    // 전송 계층 설정 (Tokio 기반 TCP)
    let tcp_transport = TokioTcpTransport::new();

    // Noise 프로토콜 설정
    let noise_keys = noise::Keypair::<noise::X25519Spec>::new()
        .into_authentic(&local_key)
        .expect("노이즈 키 생성 실패");

    let transport = tcp_transport
        .upgrade(upgrade::Version::V1)
        .authenticate(noise::NoiseConfig::xx(noise_keys).into_authenticated())
        .multiplex(yamux::YamuxConfig::default())
        .boxed();

    // DNS 설정
    let transport = TokioDnsConfig::system(transport).await.unwrap();

    // 네트워크 동작 정의
    #[derive(NetworkBehaviour)]
    #[behaviour(out_event = "MyBehaviourEvent")]
    struct MyBehaviour {
        identify: IdentifyBehaviour,
        // 필요한 다른 프로토콜 추가 가능
    }

    #[derive(Debug)]
    enum MyBehaviourEvent {
        Identify(IdentifyEvent),
        // 다른 프로토콜 이벤트 추가 가능
    }

    impl From<IdentifyEvent> for MyBehaviourEvent {
        fn from(event: IdentifyEvent) -> Self {
            MyBehaviourEvent::Identify(event)
        }
    }

    // MyBehaviour 인스턴스 생성
    let behaviour = MyBehaviour {
        identify: IdentifyBehaviour::new(IdentifyConfig::new(
            "/donghae/1.0.0".into(),
            local_key.public(),
        )),
    };

    // Swarm 생성
    let mut swarm = Swarm::new(transport, behaviour, local_peer_id);

    // 리스닝 주소 설정
    Swarm::listen_on(&mut swarm, "/ip4/0.0.0.0/tcp/0".parse().unwrap()).unwrap();

    // Swarm 이벤트 처리 루프
    tokio::spawn(async move {
        while let Some(event) = swarm.next().await {
            match event {
                SwarmEvent::NewListenAddr { address, .. } => {
                    info!("노드가 리스닝 중인 주소: {:?}", address);
                }
                SwarmEvent::Behaviour(event) => match event {
                    MyBehaviourEvent::Identify(event) => {
                        info!("Identify 이벤트: {:?}", event);
                    }
                    // 다른 이벤트 처리 추가 가능
                },
                _ => {}
            }
        }
    });
}
