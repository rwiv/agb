# TASK 1-2: 소스 코드 내 모든 `transformers` 참조를 `transformer`로 업데이트

## 1. 개요
`src/transformer`로 디렉토리 이름을 변경한 후, 모든 소스 코드에서 해당 모듈을 참조하는 임포트 경로 및 관련 변수명을 업데이트합니다.

## 2. 세부 작업 지침
- `src/main.rs`: `mod transformers;` 구문을 `mod transformer;`로 변경.
- `src/builder/core.rs`: `crate::transformers::*` 참조를 `crate::transformer::*`로 업데이트.
- `src/builder/config.rs`: `BuildTarget` 설정 관련 참조가 있다면 업데이트.
- `src/transformer/mod.rs` 및 하위 모듈(`base.rs`, `factory.rs`, `providers/`)에서 내부적으로 `crate::transformers`를 참조하고 있다면 모두 `crate::transformer`로 변경.
- 필요한 경우 변수명(`transformer` 등)도 일관되게 단수형으로 유지합니다.

## 3. 성공 기준
- 모든 Rust 파일에서 `transformers` 모듈에 대한 참조가 `transformer`로 성공적으로 업데이트되어야 합니다.
- 전체 빌드(`cargo build`)가 오류 없이 완료되어야 합니다.
