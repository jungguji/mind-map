# Mind Map - 아키텍처 문서

## 개요

Mind Map은 Rust + WebAssembly로 구현된 코어 로직과 JavaScript로 구현된 UI 레이어로 구성된 하이브리드 웹 애플리케이션입니다.

## 시스템 구조

```
┌─────────────────────────────────────────────────────┐
│                   index.html                        │
│                  (HTML 구조)                        │
└─────────────────────────────────────────────────────┘
                        │
          ┌─────────────┴─────────────┐
          │                           │
┌─────────▼─────────┐       ┌────────▼────────┐
│  styles/main.css  │       │  scripts/app.js │
│   (스타일시트)     │       │  (메인 로직)    │
└───────────────────┘       └─────────┬───────┘
                                      │
                    ┌─────────────────┼─────────────────┐
                    │                 │                 │
          ┌─────────▼──────┐  ┌──────▼──────┐  ┌──────▼──────┐
          │ event-         │  │ ui-         │  │ utils.js    │
          │ handlers.js    │  │ controller  │  │ (유틸리티)  │
          │ (이벤트 처리)  │  │ (UI 컨트롤) │  └─────────────┘
          └────────┬───────┘  └──────┬──────┘
                   │                 │
                   └────────┬────────┘
                            │
                   ┌────────▼────────┐
                   │  pkg/           │
                   │  mind_map.js    │
                   │  (Wasm 바인딩)  │
                   └────────┬────────┘
                            │
                   ┌────────▼────────┐
                   │  Rust/Wasm      │
                   │  Core Logic     │
                   │  (노드 관리)    │
                   └─────────────────┘
```

## 레이어 구조

### 1. Presentation Layer (HTML/CSS)
- **index.html**: DOM 구조 정의
- **styles/main.css**: 모든 스타일 정의 (반응형 디자인 포함)

### 2. Application Layer (JavaScript)
- **app.js**: 애플리케이션 초기화 및 모듈 조정
- **event-handlers.js**: 사용자 입력 이벤트 처리 (마우스, 터치, 키보드)
- **ui-controller.js**: UI 컴포넌트 제어 (인라인 편집, 버튼, Info 섹션)
- **utils.js**: 공통 유틸리티 함수

### 3. Core Layer (Rust/WebAssembly)
- **src/node.rs**: 노드 데이터 구조
- **src/mind_map.rs**: 마인드맵 비즈니스 로직
- **src/app.rs**: WebAssembly 바인딩 및 렌더링 로직

## 데이터 흐름

### 사용자 입력 → 화면 업데이트 플로우

```
사용자 이벤트
    │
    ▼
event-handlers.js (이벤트 캡처 및 좌표 변환)
    │
    ▼
MindMapApp (Wasm) (상태 업데이트 및 렌더링)
    │
    ▼
Canvas API (화면 렌더링)
```

### 노드 편집 플로우

```
더블클릭 이벤트
    │
    ▼
event-handlers.js (nodeDoubleClick 커스텀 이벤트 발생)
    │
    ▼
ui-controller.js (편집 UI 표시)
    │
    ▼
사용자 입력
    │
    ▼
MindMapApp.update_selected_text() (Wasm)
    │
    ▼
Canvas 리렌더링
```

## 모듈 간 의존성

```
app.js
  ├─→ utils.js
  ├─→ event-handlers.js
  │     └─→ utils.js
  ├─→ ui-controller.js
  └─→ pkg/mind_map.js (Wasm)

event-handlers.js
  └─→ ui-controller.js (커스텀 이벤트를 통한 통신)
```

## 핵심 설계 패턴

### 1. Module Pattern
각 JavaScript 파일은 ES6 모듈로 구성되어 관심사를 명확히 분리합니다.

### 2. Event-Driven Architecture
- DOM 이벤트 → event-handlers.js
- 커스텀 이벤트 (nodeDoubleClick) → ui-controller.js
- Wasm 상태 변경 → 자동 리렌더링

### 3. Separation of Concerns
- **표현 (Presentation)**: HTML, CSS
- **제어 (Control)**: JavaScript 모듈들
- **비즈니스 로직 (Business Logic)**: Rust/WebAssembly

### 4. Utility-First Helpers
공통 함수는 utils.js에 집중하여 코드 재사용성을 높입니다.

## 성능 최적화 전략

### 1. WebAssembly 활용
- 노드 계산 및 렌더링 로직을 Rust로 구현하여 성능 향상
- JavaScript는 이벤트 처리와 UI 제어에만 집중

### 2. 모듈 번들링
- ES6 모듈 사용으로 필요한 코드만 로드
- 브라우저의 네이티브 모듈 지원 활용

### 3. 반응형 디자인
- CSS 미디어 쿼리로 모바일/데스크톱 대응
- 터치 이벤트 별도 처리

## 확장성 고려사항

### 새로운 기능 추가 시

1. **새로운 이벤트 타입**: `event-handlers.js`에 핸들러 추가
2. **새로운 UI 컴포넌트**: `ui-controller.js`에 컨트롤러 추가
3. **새로운 유틸리티**: `utils.js`에 함수 추가
4. **새로운 비즈니스 로직**: Rust 코어 로직 수정

### 모듈화의 장점

- 각 파일의 책임이 명확하여 유지보수 용이
- 테스트 작성 시 개별 모듈 단위로 테스트 가능
- 팀 협업 시 충돌 최소화
- 코드 재사용성 향상

## 브라우저 호환성

- ES6 모듈 지원 필요 (Chrome 61+, Firefox 60+, Safari 11+)
- WebAssembly 지원 필요 (대부분의 최신 브라우저)
- Touch Events API (모바일 지원)
- Canvas API (모든 모던 브라우저)

## 보안 고려사항

- XSS 방지: 사용자 입력은 텍스트로만 처리 (innerHTML 미사용)
- CSP (Content Security Policy) 적용 가능
- CORS 설정 불필요 (정적 파일 제공)
