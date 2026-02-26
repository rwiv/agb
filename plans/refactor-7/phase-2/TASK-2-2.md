# TASK 2-2: `src/transformer/gemini.rs`에서 `src/utils/toml::json_to_toml`을 사용하도록 수정 및 정리

## 1. 개요
`src/transformer/gemini.rs`에서 로컬에 정의되어 있던 `json_to_toml` 함수를 제거하고, `src/utils/toml::json_to_toml`을 사용하도록 수정합니다.

## 2. 작업 내용
- `src/transformer/gemini.rs` 상단에 `use crate::utils::toml::json_to_toml;`를 추가합니다.
- `src/transformer/gemini.rs` 내의 `json_to_toml` 함수 정의 및 관련 테스트 중 `json_to_toml`만 테스트하는 내용을 제거합니다. (단, `GeminiTransformer`의 최종 변환 테스트는 유지)
- `GeminiTransformer` 내의 `crate::core::ResourceData` 경로 참조를 `ResourceData`로 수정합니다.

## 3. 검증 전략
- **빌드 검증:** `cargo check` 실행.
- **유닛 테스트:** `src/transformer/gemini.rs` 내의 유닛 테스트를 통해 `GeminiTransformer`의 기능이 여전히 정상 작동하는지 확인합니다.
