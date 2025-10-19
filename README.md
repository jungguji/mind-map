# Mind Map - Rust WebAssembly

간단한 웹 기반 마인드맵 애플리케이션입니다. Rust와 WebAssembly로 구현되었습니다.

## 기능

- Root 노드 생성 및 관리
- 자식 노드 추가
- 노드 텍스트 편집
- 드래그로 노드 위치 이동
- Canvas 기반 렌더링

## 필요 사항

이 프로젝트를 빌드하려면 다음이 필요합니다:

1. **Rustup 및 Rust**: Homebrew의 Rust 대신 rustup을 사용해야 합니다
2. **wasm-pack**: WebAssembly 빌드 도구

## 설치 방법

### 1. Homebrew Rust 제거 (이미 설치된 경우)

```bash
brew uninstall rust
```

### 2. Rustup 설치

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

설치 후 쉘을 재시작하거나:

```bash
source ~/.cargo/env
```

### 3. wasm32 타겟 추가

```bash
rustup target add wasm32-unknown-unknown
```

### 4. wasm-pack 설치

```bash
cargo install wasm-pack
```

## 빌드 및 실행

### 1. 프로젝트 빌드

```bash
wasm-pack build --target web
```

### 2. 로컬 서버 실행

빌드 후 간단한 HTTP 서버로 실행:

```bash
# Python 3를 사용하는 경우
python3 -m http.server 8080

# 또는 Node.js의 http-server 사용
npx http-server -p 8080
```

### 3. 브라우저에서 열기

브라우저에서 `http://localhost:8080`로 접속합니다.

## 사용 방법

1. **노드 선택**: 노드를 클릭하면 선택됩니다 (선택된 노드는 초록색으로 표시)
2. **노드 이동**: 노드를 드래그하여 원하는 위치로 이동할 수 있습니다
3. **자식 노드 추가**:
   - 노드를 선택한 후
   - 입력 필드에 텍스트 입력
   - "자식 노드 추가" 버튼 클릭 또는 Enter 키
4. **노드 텍스트 수정**:
   - 노드를 선택한 후
   - 입력 필드에 새 텍스트 입력
   - "선택한 노드 수정" 버튼 클릭

## 프로젝트 구조

```
mind_map/
├── Cargo.toml          # Rust 프로젝트 설정
├── src/
│   ├── lib.rs          # 메인 Rust 코드 (WebAssembly)
│   └── main.rs         # (사용하지 않음)
├── index.html          # 웹 인터페이스
└── pkg/               # 빌드된 WebAssembly 파일 (빌드 후 생성)
```

## 기술 스택

- **Rust**: 핵심 로직 및 데이터 구조
- **WebAssembly**: 브라우저에서 실행
- **wasm-bindgen**: Rust와 JavaScript 간 인터페이스
- **web-sys**: 웹 API 바인딩 (Canvas, DOM 이벤트 등)
- **Canvas API**: 그래픽 렌더링

## 라이선스

MIT
