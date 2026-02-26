# TASK 2-1: `src/utils/toml.rs` 생성 및 `json_to_toml` 함수 이동

## 1. 개요
`src/transformer/gemini.rs`에 포함되어 있던 `json_to_toml` 함수를 유틸리티 성격에 맞게 `src/utils/toml.rs`로 이동하고, 공용으로 사용할 수 있도록 구성합니다.

## 2. 작업 내용
- `src/utils/toml.rs` 파일을 생성합니다.
- `src/transformer/gemini.rs`에서 `json_to_toml` 함수를 해당 파일로 이동합니다.
- `src/utils/mod.rs`에서 `toml` 모듈을 `pub`으로 선언합니다.
- `json_to_toml` 함수에 필요한 `anyhow`, `serde_json`, `toml` 의존성을 `src/utils/toml.rs` 상단에 추가합니다.

## 3. 검증 전략
- **빌드 검증:** `cargo check` 실행.
- **유닛 테스트:** `src/utils/toml.rs` 내에 `json_to_toml`에 대한 테스트를 추가하고 통과하는지 확인합니다.
