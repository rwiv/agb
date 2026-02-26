# Task 3.2: 라이브러리 기반 통합 테스트 추가

## 1. 개요 (Overview)
`agb`가 라이브러리 구조로 변경됨에 따라, `std::process::Command`를 통한 바이너리 실행 없이도 `App` 구조체를 직접 활용하여 핵심 로직을 테스트할 수 있음을 검증하는 통합 테스트를 추가합니다.

## 2. 작업 상세 (Implementation Details)

### 2.1. `tests/app_integration_test.rs` 생성
다음과 같은 방식으로 `App`을 직접 사용하는 테스트 코드를 작성합니다.

```rust
use agb::app::{App, AppContext, Cli, Commands};
use tempfile::tempdir;
use std::fs;

#[test]
fn test_app_build_integration() {
    let temp_dir = tempdir().unwrap();
    let root = temp_dir.path();
    // ... (Fixture 설정)

    let app = App::new();
    let cli = Cli {
        command: Commands::Build { config: Some(config_path.to_string()) },
    };
    
    // 바이너리 실행 없이 로직 호출
    app.run(cli).expect("App run failed");

    // 결과물 검증
    assert!(root.join("commands/foo.toml").exists());
}
```

## 3. 검증 방법 (Verification)
- `cargo test --test app_integration_test` 명령을 실행하여 바이너리 호출 없이도 빌드/동기화 로직이 정상 작동하는지 확인합니다.
