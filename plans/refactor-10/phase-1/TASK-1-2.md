# Task 1.2: `src/app` 모듈 생성 및 `AppContext` 정의

## 1. 개요 (Overview)
`src/main.rs`의 중복되는 초기화 로직을 `AppContext` 구조체로 이동시키고, `src/app/mod.rs`를 생성하여 통합 관리합니다.

## 2. 작업 상세 (Implementation Details)

### 2.1. `src/app/mod.rs` 생성
다음 구조를 가지는 `AppContext`를 정의합니다. `loader` 모듈에 있던 `load_registry_from_config` 로직을 이 모듈의 `init` 내부로 흡수하여 설정 기반의 리소스 구성을 담당하게 합니다.

```rust
use crate::core::{Config, Registry, BuildTarget};
use crate::transformer::Transformer;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

pub struct AppContext {
    pub config: Config,
    pub registry: Registry,
    pub transformer: Box<dyn Transformer>,
    pub source_dir: PathBuf,
    pub output_dir: PathBuf,
}

impl AppContext {
    pub fn init(config_file: &str) -> anyhow::Result<Self> {
        // ... (생략: 경로 검증 및 Config 로드)

        // 설정으로부터 식별자 추출 (기존 loader::load_registry_from_config 로직 이동)
        let mut target_identifiers = HashSet::new();
        if let Some(cmds) = &cfg.resources.commands {
            target_identifiers.extend(cmds.clone());
        }
        // ... (agents, skills 추가)

        // loader 호출 (Config 의존성이 제거된 새로운 API 사용)
        let registry = crate::loader::load_registry(&source_dir, &target_identifiers, cfg.target, &exclude)?;
        
        // ... (나머지 초기화)
    }
}
```

### 2.2. `src/loader/mod.rs` 리팩토링
- `load_registry_from_config` 함수를 제거합니다.
- 대신 `Config` 객체에 의존하지 않는 범용적인 `load_registry` 함수를 제공하거나 `ResourceLoader`를 `pub(crate)`로 노출합니다.

## 3. 검증 방법 (Verification)
- `cargo build` 명령을 통해 컴파일 에러가 발생하지 않는지 확인합니다.
- `src/app/mod.rs` 내부 로직이 `main.rs`의 기존 초기화 로직과 동일하게 작동하는지 검토합니다.
