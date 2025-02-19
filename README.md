# Donghae

**Donghae**는 각 가정의 Mac을 활용해 신뢰할 수 있는 CoreML(뉴럴 엔진) 서비스를 제공하는, Swift 기반 블록체인 네트워크입니다.  
"동해(東海)"는 동쪽 바다의 넓고 깊은 이미지를 상징하며, Donghae는 집에 있는 Mac의 CoreML 기능을 분산형으로 통합하여, 높은 확장성과 신뢰성을 갖춘 플랫폼으로 발전하는 것을 목표로 합니다.

---

## 목차

1. [프로젝트 개요](#프로젝트-개요)  
2. [주요 변경 사항](#주요-변경-사항)  
3. [주요 특징](#주요-특징)  
4. [NAT 트래버설 및 libp2p 전략](#nat-트래버설-및-libp2p-전략)  
5. [플랫폼 및 CoreML 서비스 통합](#플랫폼-및-coreml-서비스-통합)  
6. [Neural Engine Optimized Consensus with PoS](#neural-engine-optimized-consensus-with-pos)  
7. [추가 아이디어](#추가-아이디어)  
8. [활용 오픈 소스](#활용-오픈-소스)  
9. [테스트 및 배포 전략](#테스트-및-배포-전략)  
10. [요약](#요약)

---

## 1. 프로젝트 개요

Donghae는 Apple Silicon 기반 Mac 환경에서 CoreML 모델을 이용한 추론 서비스를 효율적으로 제공하기 위해 설계된 블록체인 플랫폼입니다.  
각 가정의 Mac이 노드로 참여하여 머신러닝 추론 결과를 블록체인에 기록하고, 분산 노드들 간 안전하고 빠른 데이터 교환을 가능하게 합니다.  
Swift(Vapor)와 libp2p를 도입함으로써 P2P 네트워크를 직관적이고 안정적으로 구현합니다.

---

## 2. 주요 변경 사항

1. **Zig 언어 미사용 → Swift 전환**  
   - Swift와 Vapor를 활용하여 전체 시스템을 재구현. Apple Silicon 생태계에서 최적의 성능을 낼 수 있도록 설계합니다.

2. **libp2p 통합**  
   - NAT 트래버설, 피어 디스커버리, 트랜스포트(QUIC, TCP 등)를 libp2p를 통해 처리하여 P2P 네트워크를 더욱 자동화하고 안정화합니다.

3. **Apple Silicon 최적화**  
   - Swift의 Metal, Accelerate, CoreML 등을 활용해 뉴럴 엔진 성능을 최대화합니다.

4. **QUIC 및 gRPC 통신**  
   - Swift-NIO 기반 QUIC 프로토콜과 gRPC를 활용하여 빠르고 안전한 노드 간 통신을 구현합니다.

5. **NAT 트래버설 개선**  
   - libp2p(AutoNAT, Hole Punch 등)와 miniupnpc, libnatpmp를 혼합 운용해, 다양한 가정용 라우터 환경에서 자동화된 포트 매핑 및 NAT 우회를 지원합니다.

---

## 3. 주요 특징

- **Swift 기반 안전하고 효율적인 코드베이스**  
  - Apple Silicon 환경에 최적화된 코드를 제공하며, macOS 생태계와의 시너지 효과를 높입니다.

- **libp2p 통합 P2P 네트워크**  
  - 피어 디스커버리, NAT 트래버설, 메시지 라우팅을 libp2p 모듈(AutoNAT, Hole Punch, DHT 등)에 의존함으로써 복잡도를 줄이고 확장성을 확보합니다.

- **최적화된 네트워크 통신**  
  - Swift-NIO 및 QUIC 프로토콜을 통해 빠르고 안전한 연결을 제공합니다.  
  - gRPC를 활용해 노드 간 API 호출 및 CoreML 모델 관리 기능을 쉽게 확장할 수 있습니다.

- **Neural Engine Optimized Consensus with PoS**  
  - CoreML 추론 결과와 함께 PoS(Proof of Stake) 기반 검증을 수행해, 정확하고 빠른 결과에 대해 경제적 보상을 지급합니다.

- **자동 NAT 트래버설**  
  - libp2p의 AutoNAT 모듈로 NAT 환경을 자동 탐지하고, Hole Punch를 통해 포트를 직접 열기 어려운 상황에서 피어 간 직접 연결을 시도합니다.  
  - 호환되지 않는 라우터 환경에서는 miniupnpc(UPnP) 및 libnatpmp(NAT-PMP)를 통해 포트 매핑을 수행해 연결 성공률을 높입니다.

- **Solana 벤치마킹**  
  - Solana의 빠른 블록 생성과 고성능 합의 구조를 참조해, QUIC 기반 확장성과 처리량 개선에 집중합니다.

- **GUI 기반 사용자 친화성**  
  - Vapor 웹 대시보드 또는 SwiftUI 기반 GUI를 제공해, 노드 상태와 CoreML 작업 진행 상황을 시각적으로 모니터링·관리할 수 있습니다.

---

## 4. NAT 트래버설 및 libp2p 전략

Donghae는 libp2p 모듈을 통해 다양한 NAT 환경에서 자동으로 네트워크 연결을 시도합니다.  
- **AutoNAT**: NAT 환경을 자동 식별하고, 외부 IP를 확인해 직접 연결(Hole Punch) 또는 릴레이 노드를 사용할지를 결정합니다.  
- **Hole Punch**: 포트 포워딩 없이도 사설 IP 대역 간 직접 연결을 시도하여, 별도의 설정 부담을 크게 줄여 줍니다.  
- **miniupnpc & libnatpmp**: 라우터가 UPnP/NAT-PMP를 지원할 경우, libp2p 이전에 Swift 레벨에서 포트 매핑을 시도해 중계 노드를 거치지 않고도 안정적인 연결을 수립합니다.

---

## 5. 플랫폼 및 CoreML 서비스 통합

Donghae는 Apple Silicon이 제공하는 CoreML 뉴럴 엔진을 집중적으로 활용하여, 분산 머신러닝 추론 서비스를 구현합니다.

- **CoreML 서비스 통합**  
  - Swift로 작성된 CoreML 모델 또는 사전 학습 모델을 노드가 직접 실행하고, 추론 결과를 블록체인에 기록합니다.  
- **동적 작업 분배**  
  - 빠른 노드는 더 많은 추론 작업을, 느린 노드는 적절한 양의 작업을 수행하도록 조정해 전체 처리량을 극대화합니다.  
- **결과 검증 및 기록**  
  - 노드가 제출한 추론 결과는 블록체인에 기록되고, PoS 기반 합의를 통해 정확도가 높은 결과에 보상이 분배됩니다.

---

## 6. Neural Engine Optimized Consensus with PoS

동해는 단순 토큰 스테이킹을 넘어, CoreML 추론 결과 품질을 고려한 독창적인 합의 구조를 갖추고 있습니다.

1. **PoS와 CoreML 결합**  
   - 일정량의 토큰을 스테이킹하고, 추론 결과와 증명(서명·해시)을 제출합니다.  
   - 다른 노드들이 결과를 검증해 정확하면 보상을 부여합니다.  
2. **공정성 및 인센티브 설계**  
   - 처리 속도뿐 아니라 정확도도 고려해, 성능이 다소 낮은 노드도 성실하게 기여하면 꾸준히 이익을 얻을 수 있도록 설계합니다.  
3. **동적 작업 스케줄링**  
   - 네트워크 전체에서 처리해야 할 추론 작업이 큐에 쌓이며, 노드별 성능 지표에 따라 작업이 고르게 분산됩니다.

---

## 7. 추가 아이디어

- **IPFS 연계**  
  - CoreML 모델이나 대규모 데이터를 IPFS에 저장하고, libp2p를 통해 빠르게 배포하는 구조를 추가할 수 있습니다.
- **스마트 컨트랙트로 모델 버전 관리**  
  - 모델 업데이트와 버전을 블록체인에서 자동으로 관리해, 노드들이 항상 최신 모델을 활용하도록 지원합니다.
- **연합 학습(Federated Learning) 지원**  
  - 각 노드가 학습한 부분 파라미터를 모으고, 중앙 서버 없이 전체 모델을 개선하는 분산 학습 체계를 고려할 수 있습니다.
- **Solana식 병렬화**  
  - Solana를 참고해, 트랜잭션과 상태를 병렬 처리해 높은 TPS(초당 트랜잭션 처리량)를 달성하는 방안을 검토합니다.

---

## 8. 활용 오픈 소스

Donghae 프로젝트에서 사용되는 주요 오픈 소스 기술들은 다음과 같습니다:

- **[Swift & Vapor](https://www.vapor.codes/)**  
  - 웹 서버 및 REST/gRPC API 구현을 위한 Swift 기반 프레임워크  
- **[libp2p](https://libp2p.io/)**  
  - P2P 네트워킹, NAT 트래버설, 피어 디스커버리, 암호화·보안 등 기능을 통합적으로 제공  
- **[miniupnpc](http://miniupnp.free.fr/)**  
  - UPnP 프로토콜을 통한 가정용 라우터 포트 매핑  
- **[libnatpmp](https://miniupnp.tuxfamily.org/libnatpmp.html)**  
  - NAT-PMP 프로토콜로 간소화된 포트 매핑 지원  
- **[CoreML & Accelerate](https://developer.apple.com/documentation/coreml)**  
  - Apple Silicon 환경에서 고성능 뉴럴 엔진 및 수학 연산 최적화  
- **[Swift-NIO QUIC](https://github.com/apple/swift-nio-transport-services)**  
  - QUIC 기반 저지연·고성능 통신을 위한 SwiftNIO 모듈

---

## 9. 테스트 및 배포 전략

1. **유닛 테스트**  
   - Swift `XCTest`로 네트워크, NAT 트래버설, 합의 알고리즘, CoreML 추론 로직 등을 모듈 단위로 철저히 검증합니다.  
2. **통합 테스트**  
   - 여러 대의 libp2p 노드를 시뮬레이션 환경에서 구성하고, NAT 우회 성공률, 트랜잭션 처리 성능, 모델 추론 정확도 등을 점검합니다.  
3. **배포**  
   - Vapor 기반 백엔드를 Docker(Apple Silicon 지원) 컨테이너나 맥OS 전용 앱 형태로 배포할 수 있습니다.  
   - `brew` 또는 스크립트를 통해 간편 설치가 가능하며, CLI 및 GUI 툴로 노드 실행과 모니터링이 가능합니다.

---

## 10. 요약

Donghae는 집에 있는 Mac을 분산형 CoreML 노드로 전환해, 탈중앙화된 머신러닝 서비스와 고성능 블록체인 네트워크를 융합하는 프로젝트입니다.  
- **Swift(Vapor) + libp2p** 기반으로 P2P 기능을 강력하고 단순화된 방식으로 제공하고,  
- **Apple Silicon**의 뉴럴 엔진 성능과 **CoreML** 기술을 결합해 효율적인 분산 추론을 실현하며,  
- **PoS 합의**에 CoreML 추론 결과의 정확도를 접목해 정확하고 빠른 결과를 네트워크에 기여한 노드에 인센티브를 부여합니다.  

이를 통해 Donghae는 **Solana식 확장성**과 **Apple Silicon 최적화**를 모두 추구하여, Mac 사용자들에게 친숙하고, 분산 머신러닝 생태계에 기여하는 새로운 블록체인 플랫폼을 만들어 나가고자 합니다.
