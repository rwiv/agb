# Task 2.2: `main.rs` 축소 및 `App::run` 호출

## 1. 개요 (Overview)
`src/main.rs`의 모든 비즈니스 로직을 제거하고, `clap`을 이용한 CLI 인자 파싱 결과만 `App::run`에 전달하도록 수정합니다.

## 2. 작업 상세 (Implementation Details)

### 2.1. `src/main.rs` 수정
`src/main.rs`의 `main` 함수를 다음과 같이 축소합니다.

```rust
use clap::Parser;
use agb::app::{App, Cli};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let app = App::new();
    app.run(cli)
}
```

### 2.2. `Cli` 구조체 가시성 변경
`src/app/mod.rs`로 이동시킨 `Cli`와 `Commands` 열거형이 `main.rs`에서 접근 가능하도록 `pub` 키워드를 추가합니다.

## 3. 검증 방법 (Verification)
- `cargo run -- build` 명령을 실행하여 정상적으로 기존 빌드 동작이 수행되는지 확인합니다.
- `cargo run -- sync` 명령을 실행하여 정상적으로 동기화 동작이 수행되는지 확인합니다.
- 빌드 결과물이 리팩토링 전과 동일하게 생성되는지 `diff` 등으로 검증합니다.
