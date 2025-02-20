package main

/*
#include <stdint.h>
*/
import "C"

import (
    "fmt"
    "sync"

    libp2p "github.com/libp2p/go-libp2p"
    "github.com/libp2p/go-libp2p/core/host"
    "github.com/libp2p/go-libp2p/core/peer"
)

var (
    hostInstance *Node
    mu           sync.Mutex
)

type Node struct {
    Host host.Host
}

//export StartHost
func StartHost() {
    mu.Lock()
    defer mu.Unlock()

    if hostInstance != nil {
        fmt.Println("Host already started.")
        return
    }

    // libp2p 호스트 생성
    h, err := libp2p.New()
    if err != nil {
        panic(err)
    }
    hostInstance = &Node{
        Host: h,
    }

    fmt.Println("libp2p host started!")
    // Pretty() 메서드는 없어졌으므로, 그냥 ID() 자체를 문자열로 출력 가능
    fmt.Printf("Peer ID: %s\n", h.ID())
    fmt.Printf("Listen Addrs: %v\n", h.Addrs())
}

//export StopHost
func StopHost() {
    mu.Lock()
    defer mu.Unlock()

    if hostInstance == nil {
        fmt.Println("No running host to stop.")
        return
    }
    hostInstance.Host.Close()
    hostInstance = nil
    fmt.Println("libp2p host stopped!")
}

//export PrintPeers
func PrintPeers() {
    mu.Lock()
    defer mu.Unlock()

    if hostInstance == nil {
        fmt.Println("Host not started.")
        return
    }

    peers := hostInstance.Host.Network().Peers()
    fmt.Printf("Connected peers: %d\n", len(peers))
    for _, p := range peers {
        fmt.Printf("- %s\n", peer.ID.String(p))
    }
}

// c-shared 빌드를 위해 main() 함수가 필요 (빈 함수여도 상관없음)
func main() {}
