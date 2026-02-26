# Task 1-2: Target-Aware Metadata Merge Logic

## 1. 개요
추출된 두 개의 메타데이터 객체를 현재 `BuildTarget` 규칙에 따라 병합하는 핵심 비즈니스 로직을 구현합니다.

## 2. 상세 요구사항
- **함수 위치**: `src/resource/loader/parser.rs` 내부 또는 별도 유틸리티.
- **로직 흐름**:
    1. `external` 객체가 `Value::Object`가 아니면 병합을 중단하고 `base` 유지.
    2. `external` 객체의 모든 키-값 쌍을 순회하며 `base`에 `insert` (Shallow merge).
    3. 현재 `target` (GeminiCli, ClaudeCode, OpenCode)에 해당하는 섹션 키(`"gemini"`, `"claude"`, `"opencode"`)를 결정.
    4. `external` 내에 해당 섹션 키가 존재하고 그 값이 객체라면, 그 내부의 모든 필드를 다시 `base`에 `insert` 하여 최종 오버라이트 수행.
    5. **중요**: 최종 `base` 객체에서 모든 타겟 예약어 키(`"gemini"`, `"claude"`, `"opencode"`)를 `remove` 하여 결과물을 정돈함.

## 3. 주의사항
- **Shallow Merge**: `base.as_object_mut().unwrap().insert(k, v)` 방식을 사용하여 최상위 키를 통째로 교체해야 함. 재귀적 병합은 고려하지 않음.
- **우선순위**: `외부 파일의 일반 필드` < `외부 파일의 타겟 섹션 내부 필드` 순으로 우선순위를 가짐.

## 4. 테스트 시나리오
- `.md`에 `model: A`가 있고, 외부 파일 `gemini: { model: B }`가 있을 때 최종 결과가 `B`인지 확인.
- 외부 파일의 `claude` 섹션 내용이 `GeminiCli` 타겟 빌드 시 무시되는지 확인.
- 병합 후 최종 객체에 `"gemini"` 같은 섹션 키가 남아있지 않은지 확인.
