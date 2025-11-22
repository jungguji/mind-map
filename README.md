# Mind Map - Rust WebAssembly

간단한 웹 기반 마인드맵 애플리케이션입니다. Rust와 WebAssembly로 구현되었습니다.

## 기능

- **노드 관리**: Root 노드 생성 및 자식 노드 추가
- **멀티 선택**: 드래그 박스로 여러 노드 동시 선택 및 이동
- **인라인 편집**: 노드 더블클릭으로 텍스트 즉시 수정
- **드래그 앤 드롭**: 노드를 드래그하여 자유롭게 위치 이동
- **캔버스 패닝**:
  - 데스크톱: Space 키를 누른 채로 드래그
  - 모바일: 2손가락으로 드래그
- **노드 삭제**: Delete 키로 선택된 노드 삭제 (Root 제외)
- **모바일 지원**: 터치 제스처 및 반응형 디자인
- **Canvas 기반 렌더링**: 고성능 그래픽 렌더링

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

### 기본 조작

- **노드 선택**: 노드를 클릭하여 선택 (선택된 노드는 초록색, 테두리 두껍게 표시)
- **멀티 선택**: 빈 공간을 드래그하여 박스로 여러 노드를 선택
- **노드 이동**: 선택된 노드를 드래그하여 이동 (여러 노드 선택 시 함께 이동)
- **노드 편집**: 노드를 더블클릭하여 텍스트 즉시 수정

### 캔버스 조작

- **데스크톱**: Space 키를 누른 채로 드래그하여 캔버스 이동
- **모바일**: 2손가락으로 드래그하여 캔버스 이동

### 노드 관리

- **자식 노드 추가**:
  1. 노드를 하나만 선택
  2. 상단 입력 필드에 텍스트 입력
  3. "자식 노드 추가" 버튼 클릭 또는 Enter 키

- **노드 텍스트 수정**:
  - 방법 1: 노드를 더블클릭하여 인라인 편집
  - 방법 2: 노드 선택 후 입력 필드에 텍스트 입력 → "선택한 노드 수정" 버튼 클릭

- **노드 삭제**:
  - 노드 선택 후 Delete 키 입력
  - 또는 "선택한 노드 삭제" 버튼 클릭
  - Root 노드는 삭제할 수 없습니다
  - 여러 노드를 동시에 삭제할 수 있습니다

## 프로젝트 구조

```
mind-map/
├── Cargo.toml              # Rust 프로젝트 설정
├── index.html              # 웹 인터페이스 (간소화된 HTML)
├── styles/
│   └── main.css           # 전체 스타일시트 (반응형 디자인)
├── scripts/
│   ├── app.js             # 메인 애플리케이션 로직
│   ├── event-handlers.js  # 이벤트 핸들러 (마우스/터치/키보드)
│   ├── ui-controller.js   # UI 컨트롤러 (편집, 버튼 등)
│   └── utils.js           # 유틸리티 함수
├── src/
│   ├── lib.rs             # Rust 진입점
│   ├── node.rs            # 노드 데이터 구조
│   ├── mind_map.rs        # 마인드맵 비즈니스 로직
│   └── app.rs             # WebAssembly 바인딩 및 렌더링
├── docs/
│   ├── architecture.md    # 시스템 아키텍처 문서
│   └── file-roles.md      # 각 파일의 역할 및 가이드
└── pkg/                   # 빌드된 WebAssembly 파일 (빌드 후 생성)
```

## 기술 스택

### Backend (Rust/WebAssembly)
- **Rust**: 핵심 로직 및 데이터 구조
- **WebAssembly**: 브라우저에서 고성능 실행
- **wasm-bindgen**: Rust와 JavaScript 간 인터페이스
- **web-sys**: 웹 API 바인딩 (Canvas, DOM 이벤트 등)

### Frontend (JavaScript/CSS)
- **ES6 모듈**: 모듈화된 JavaScript 구조
- **Canvas API**: 그래픽 렌더링
- **Touch Events API**: 모바일 제스처 지원
- **반응형 디자인**: 데스크톱 및 모바일 최적화

## 개발자 문서

프로젝트 구조와 각 파일의 역할에 대한 자세한 정보는 다음 문서를 참고하세요:

- **[아키텍처 문서](docs/architecture.md)**: 시스템 구조, 데이터 흐름, 설계 패턴
- **[파일 역할 가이드](docs/file-roles.md)**: 각 파일의 상세 역할, 주요 함수, 수정 가이드

## 개발 가이드

### 코드 수정 후 재빌드

```bash
# Rust 코드 수정 시
wasm-pack build --target web

# JavaScript/CSS 수정 시
# 재빌드 불필요 (브라우저 새로고침)
```

### 새로운 기능 추가

1. **이벤트 처리 추가**: `scripts/event-handlers.js` 수정
2. **UI 컴포넌트 추가**: `scripts/ui-controller.js` 수정
3. **비즈니스 로직 추가**: `src/*.rs` 수정 후 재빌드
4. **스타일 변경**: `styles/main.css` 수정

자세한 내용은 [file-roles.md](docs/file-roles.md)를 참고하세요.

## 브라우저 호환성

- Chrome 61+
- Firefox 60+
- Safari 11+
- Edge 79+

ES6 모듈과 WebAssembly를 지원하는 모든 모던 브라우저에서 동작합니다.

## 라이선스

MIT
