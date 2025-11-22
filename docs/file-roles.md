# 파일별 역할 문서

이 문서는 Mind Map 프로젝트의 각 파일이 담당하는 역할과 주요 함수를 상세히 설명합니다.

## 📁 프로젝트 구조

```
mind-map/
├── index.html                  # HTML 구조
├── styles/
│   └── main.css               # 전체 스타일시트
├── scripts/
│   ├── app.js                 # 메인 애플리케이션 로직
│   ├── event-handlers.js      # 이벤트 핸들러
│   ├── ui-controller.js       # UI 컨트롤러
│   └── utils.js               # 유틸리티 함수
├── docs/
│   ├── architecture.md        # 아키텍처 문서
│   └── file-roles.md          # 이 문서
├── src/
│   ├── lib.rs                 # Rust 진입점
│   ├── node.rs                # 노드 데이터 구조
│   ├── mind_map.rs            # 마인드맵 비즈니스 로직
│   └── app.rs                 # WebAssembly 바인딩
└── pkg/
    └── mind_map.js            # Wasm 컴파일 결과
```

---

## 🌐 Frontend Files

### index.html

**역할**: 애플리케이션의 HTML 구조를 정의합니다.

**책임**:
- DOM 요소 정의 (캔버스, 버튼, 입력 필드 등)
- 외부 CSS 및 JavaScript 모듈 로드
- 기본 레이아웃 구조 제공

**주요 요소**:
- `<canvas id="canvas">`: 마인드맵 렌더링 영역
- `<input id="nodeText">`: 노드 텍스트 입력 필드
- `<input id="editInput">`: 인라인 편집용 입력 필드
- 버튼들: `#addNode`, `#updateNode`, `#deleteNode`
- `.info` 섹션: 사용 방법 안내

**의존성**:
- `styles/main.css`: 스타일시트
- `scripts/app.js`: 메인 JavaScript 모듈

---

### styles/main.css

**역할**: 애플리케이션의 모든 스타일을 정의합니다.

**책임**:
- 전역 스타일 설정 (리셋, 폰트, 레이아웃)
- 컴포넌트별 스타일 (버튼, 입력 필드, 캔버스)
- 반응형 디자인 (모바일/데스크톱)
- 애니메이션 및 트랜지션

**주요 스타일 그룹**:
1. **Global Styles**: 전체 페이지 레이아웃
2. **Controls**: 버튼 및 입력 필드 스타일
3. **Canvas**: 캔버스 영역 스타일
4. **Info Section**: 사용 방법 섹션
5. **Edit Input**: 인라인 편집 입력 필드
6. **Media Queries**: 반응형 스타일 (768px, 480px 브레이크포인트)

**의존성**: 없음 (독립적)

---

## 📜 JavaScript Modules

### scripts/app.js

**역할**: 애플리케이션의 메인 진입점으로, 모든 모듈을 초기화하고 조정합니다.

**책임**:
- WebAssembly 모듈 초기화
- MindMapApp 인스턴스 생성
- 캔버스 크기 설정 및 리사이즈 핸들러 등록
- 모든 이벤트 핸들러 및 UI 컨트롤러 설정
- 초기 렌더링

**주요 함수**:

#### `async function run()`
- WebAssembly 모듈을 초기화하고 애플리케이션을 시작합니다.
- 순서:
  1. `init()` - Wasm 초기화
  2. `MindMapApp` 인스턴스 생성
  3. 캔버스 리사이즈 설정
  4. UI 컴포넌트 설정
  5. 이벤트 핸들러 설정

**의존성**:
- `../pkg/mind_map.js` (Wasm 모듈)
- `./utils.js`
- `./event-handlers.js`
- `./ui-controller.js`

**내보내기**: 없음 (즉시 실행)

---

### scripts/utils.js

**역할**: 여러 모듈에서 공통으로 사용하는 유틸리티 함수를 제공합니다.

**책임**:
- 이벤트 좌표 추출
- 터치 제스처 계산
- 캔버스 크기 조정

**주요 함수**:

#### `getEventCoordinates(e, canvas)`
마우스/터치 이벤트에서 캔버스 기준 좌표를 추출합니다.

**매개변수**:
- `e`: MouseEvent | TouchEvent
- `canvas`: HTMLCanvasElement

**반환값**: `{offsetX: number, offsetY: number}`

**사용처**: event-handlers.js

---

#### `getTwoFingerDistance(e)`
2손가락 터치 제스처의 거리를 계산합니다.

**매개변수**:
- `e`: TouchEvent

**반환값**: `number` (픽셀 단위 거리)

**사용처**: event-handlers.js (패닝 감지)

---

#### `resizeCanvas(canvas, app)`
윈도우 크기에 맞춰 캔버스 크기를 동적으로 조정합니다.

**매개변수**:
- `canvas`: HTMLCanvasElement
- `app`: MindMapApp 인스턴스

**반환값**: 없음

**사용처**: app.js (리사이즈 핸들러)

**의존성**: 없음

**내보내기**: 모든 함수 (named exports)

---

### scripts/event-handlers.js

**역할**: 사용자 입력 이벤트를 처리하고 MindMapApp으로 전달합니다.

**책임**:
- 마우스 이벤트 처리 (클릭, 이동, 더블클릭)
- 터치 이벤트 처리 (탭, 드래그, 2손가락 제스처)
- 키보드 이벤트 처리 (Space, Delete 등)
- 커스텀 이벤트 발생 (nodeDoubleClick)

**주요 함수**:

#### `setupMouseEvents(canvas, app)`
마우스 이벤트 리스너를 설정합니다.

**이벤트**:
- `mousedown` → `app.handle_mouse_down()`
- `mousemove` → `app.handle_mouse_move()`
- `mouseup` → `app.handle_mouse_up()`
- `dblclick` → `app.handle_double_click()` + 커스텀 이벤트 발생

---

#### `setupTouchEvents(canvas, app)`
터치 이벤트 리스너를 설정합니다 (모바일 지원).

**특징**:
- 더블 탭 감지 (300ms 이내)
- 2손가락 패닝 지원
- 길게 누르기 감지 (500ms)
- 진동 피드백

**이벤트**:
- `touchstart` → 싱글/더블 탭, 2손가락 제스처 감지
- `touchmove` → 드래그 처리
- `touchend` → 제스처 종료

---

#### `setupKeyboardEvents(app, isEditingCallback)`
키보드 이벤트 리스너를 설정합니다.

**매개변수**:
- `app`: MindMapApp 인스턴스
- `isEditingCallback`: 편집 상태 확인 함수

**이벤트**:
- `keydown` (Space) → 패닝 모드 활성화
- `keydown` (기타) → `app.handle_key_down()`
- `keyup` (Space) → 패닝 모드 비활성화

**주의사항**:
- 인라인 편집 중이거나 nodeText 입력 필드에 포커스가 있을 때는 Space 키 무시

**의존성**:
- `./utils.js` (좌표 추출 함수)

**내보내기**: 모든 setup 함수 (named exports)

---

### scripts/ui-controller.js

**역할**: UI 컴포넌트를 제어하고 사용자 인터랙션을 관리합니다.

**책임**:
- 인라인 편집 UI 관리
- 버튼 이벤트 처리
- Info 섹션 토글 (모바일)

**주요 함수**:

#### `setupInlineEditing(canvas, app)`
노드 인라인 편집 기능을 설정합니다.

**반환값**: `{isEditing: function(): boolean}`

**내부 함수**:
- `showEditInput(coords, selectedText)`: 편집 입력 필드 표시
  - 데스크톱: 노드 위치에 표시
  - 모바일: 화면 중앙 하단에 표시
- `finishEditing(save)`: 편집 완료 및 저장

**이벤트**:
- `nodeDoubleClick` (커스텀) → `showEditInput()` 호출
- `keydown` (Enter) → 저장 후 종료
- `keydown` (Escape) → 취소 후 종료
- `blur` → 저장 후 종료

---

#### `setupButtons(app)`
버튼 클릭 이벤트를 설정합니다.

**이벤트**:
- `#addNode` 클릭 → `app.add_child_to_selected()`
- `#updateNode` 클릭 → `app.update_selected_text()`
- `#deleteNode` 클릭 → `app.delete_selected_node()`
- `#nodeText` Enter 키 → 자식 노드 추가

---

#### `setupInfoSection()`
사용 방법 섹션의 접기/펼치기 기능을 설정합니다.

**동작**:
- 모바일 (768px 이하): 기본적으로 접힌 상태
- 제목 클릭 시 토글

**의존성**: 없음

**내보내기**: 모든 setup 함수 (named exports)

---

## 🦀 Rust/WebAssembly Files

### src/lib.rs

**역할**: Rust 프로젝트의 진입점입니다.

**책임**:
- 모듈 선언
- 공개 API 정의

**모듈**:
- `mod node`: 노드 데이터 구조
- `mod mind_map`: 마인드맵 비즈니스 로직
- `mod app`: WebAssembly 바인딩

**내보내기**: `pub use app::MindMapApp`

---

### src/node.rs

**역할**: 마인드맵 노드의 데이터 구조를 정의합니다.

**주요 구조체**:
- `Node`: 개별 노드 (ID, 텍스트, 위치, 자식 노드 등)

**책임**:
- 노드 생성, 수정, 삭제
- 노드 간 관계 관리 (부모-자식)
- 노드 위치 계산

---

### src/mind_map.rs

**역할**: 마인드맵 전체의 비즈니스 로직을 담당합니다.

**주요 구조체**:
- `MindMap`: 노드 컬렉션 및 루트 노드 관리

**책임**:
- 노드 추가/수정/삭제
- 노드 검색 및 선택
- 레이아웃 계산

---

### src/app.rs

**역할**: WebAssembly 바인딩 및 JavaScript와의 인터페이스를 제공합니다.

**주요 구조체**:
- `MindMapApp`: JavaScript에서 사용할 수 있는 API

**책임**:
- Canvas 렌더링
- 이벤트 핸들링 (JavaScript → Rust)
- 상태 관리
- 드래그 앤 드롭, 선택 박스 등 인터랙션 로직

**주요 메서드**:
- `new(canvas)`: 인스턴스 생성
- `render()`: 캔버스에 렌더링
- `handle_mouse_down/move/up()`: 마우스 이벤트 처리
- `handle_pointer_*()`: 터치 이벤트 처리
- `add_child_to_selected()`: 자식 노드 추가
- `delete_selected_node()`: 선택된 노드 삭제
- `update_selected_text()`: 노드 텍스트 업데이트

---

## 📊 모듈 간 통신

### JavaScript → Wasm
```javascript
app.handle_mouse_down(e);
app.add_child_to_selected(text);
```

### Wasm → Canvas
```rust
context.fill_rect(x, y, width, height);
context.stroke_text(text, x, y);
```

### JavaScript 모듈 간 커스텀 이벤트
```javascript
// event-handlers.js
canvas.dispatchEvent(new CustomEvent('nodeDoubleClick', { detail }));

// ui-controller.js
canvas.addEventListener('nodeDoubleClick', handler);
```

---

## 🔄 수정 시 가이드

### 새로운 이벤트 타입 추가
1. `event-handlers.js`에 새 setup 함수 추가
2. `app.js`에서 해당 함수 호출
3. 필요시 Wasm 메서드 추가 (`src/app.rs`)

### 새로운 UI 컴포넌트 추가
1. `index.html`에 HTML 요소 추가
2. `styles/main.css`에 스타일 추가
3. `ui-controller.js`에 컨트롤러 함수 추가
4. `app.js`에서 초기화 함수 호출

### 유틸리티 함수 추가
1. `utils.js`에 함수 추가
2. 필요한 모듈에서 import

### 비즈니스 로직 수정
1. Rust 파일 수정 (`src/*.rs`)
2. `wasm-pack build` 재빌드
3. 필요시 JavaScript API 호출 코드 수정

---

## 📝 코딩 규칙

### JavaScript
- ES6 모듈 사용
- named exports 선호
- JSDoc 주석으로 함수 문서화
- 명확한 함수명 사용 (동사 + 명사)

### CSS
- 모바일 우선 디자인
- BEM 네이밍 사용 (선택사항)
- 미디어 쿼리는 파일 하단에 배치

### Rust
- 타입 안정성 최대한 활용
- 에러 처리 명시적으로
- pub API는 최소화

---

## 🧪 테스트 전략

### 단위 테스트
- `utils.js`: 좌표 계산 함수
- Rust 코드: `#[cfg(test)]` 모듈

### 통합 테스트
- 이벤트 → Wasm → 렌더링 플로우
- E2E 테스트 (Playwright, Cypress 등)

### 수동 테스트
- 다양한 브라우저 테스트
- 모바일 기기 테스트
- 터치 제스처 테스트
