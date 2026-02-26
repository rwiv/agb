# Task 1.1: `src/lib.rs` 생성 및 `mod` 선언 이동

## 1. 개요 (Overview)
현재 `src/main.rs`에 직접 선언된 모든 `mod`를 `src/lib.rs`로 이동하여 `agb` 프로젝트를 라이브러리 형태의 패키지 구조로 변경합니다.

## 2. 작업 상세 (Implementation Details)

### 2.1. `src/lib.rs` 생성
`src/lib.rs` 파일을 생성하고 다음 모듈들을 선언합니다.

```rust
pub mod builder;
pub mod core;
pub mod loader;
pub mod syncer;
pub mod transformer;
pub mod utils;
pub mod app; // Phase 1.2에서 추가 예정
```

### 2.2. `src/main.rs` 수정
`src/main.rs`의 상단 `mod` 선언들을 제거하고, `agb` 패키지 내부의 모듈들을 참조하도록 수정합니다.

```rust
// Remove: mod builder, mod core, ...
use agb::builder::Builder;
use agb::syncer::Syncer;
use agb::loader;
use agb::transformer;
use agb::core;
```

## 3. 검증 방법 (Verification)
- `cargo build` 명령을 통해 컴파일 에러가 발생하지 않는지 확인합니다.
- `cargo run -- build` 명령을 실행하여 정상적으로 기존 동작(설정 로드 등)이 수행되는지 확인합니다.
