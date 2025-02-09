// src/utils/mod.rs

use std::error::Error;
use hex; // hex 크레이트 임포트

// 16진수 문자열을 바이트 배열로 변환하는 함수입니다.
pub fn hex_to_bytes(hex_str: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    // 1. hex 문자열을 디코딩합니다.
    let bytes = hex::decode(hex_str)?;
    // 2. 결과를 반환합니다.
    Ok(bytes)
}

// 바이트 배열을 16진수 문자열로 변환하는 함수입니다.
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    // 1. 바이트 배열을 16진수 문자열로 인코딩합니다.
    hex::encode(bytes)
}
