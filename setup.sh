#!/bin/bash

echo "==================================="
echo "Mind Map 프로젝트 설정 스크립트"
echo "==================================="
echo ""

# Rustup 확인
if command -v rustup &> /dev/null; then
    echo "✓ Rustup이 이미 설치되어 있습니다."
else
    echo "Rustup을 설치합니다..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
    echo "✓ Rustup 설치 완료"
fi

# wasm32 타겟 추가
echo ""
echo "wasm32-unknown-unknown 타겟을 추가합니다..."
rustup target add wasm32-unknown-unknown
echo "✓ wasm32 타겟 추가 완료"

# wasm-pack 설치 확인
echo ""
if command -v wasm-pack &> /dev/null; then
    echo "✓ wasm-pack이 이미 설치되어 있습니다."
else
    echo "wasm-pack을 설치합니다..."
    cargo install wasm-pack
    echo "✓ wasm-pack 설치 완료"
fi

# 빌드
echo ""
echo "프로젝트를 빌드합니다..."
wasm-pack build --target web

if [ $? -eq 0 ]; then
    echo "✓ 빌드 성공!"
    echo ""
    echo "==================================="
    echo "설정이 완료되었습니다!"
    echo "==================================="
    echo ""
    echo "다음 명령어로 로컬 서버를 실행하세요:"
    echo ""
    echo "  python3 -m http.server 8080"
    echo ""
    echo "또는"
    echo ""
    echo "  npx http-server -p 8080"
    echo ""
    echo "그 다음 브라우저에서 http://localhost:8080 에 접속하세요."
else
    echo "✗ 빌드 실패"
    exit 1
fi
