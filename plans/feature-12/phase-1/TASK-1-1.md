# TASK 1-1: Add `contains_by_id` to `Registry`

## 개요 (Description)
의존성 리소스가 `Registry` 내에 존재하는지 리소스 식별자(`plugin:name`)와 타입을 통해 확인하는 메서드를 추가합니다.

## 수정 파일 (Files to Modify)
- `src/loader/registry.rs`

## 상세 지침 (Actionable Instructions)
1. `Registry` 구조체에 `pub fn contains_by_id(&self, r_type: ResourceType, plugin: &str, name: &str) -> bool` 메서드를 추가합니다.
2. 내부적으로 `self.resources.values()`를 순회하며 `r_type`, `plugin`, `name`이 모두 일치하는 리소스가 있는지 검사합니다.
3. (선택 사항) 성능 최적화가 필요한 경우 별도의 인덱스 맵을 고려할 수 있으나, 현재 규모에서는 순회 방식으로도 충분합니다.

## 검증 방법 (Verification)
- `src/loader/registry.rs`에 유닛 테스트를 추가하여 특정 플러그인 소속 리소스를 식별자로 찾을 수 있는지 확인합니다.
- `cargo check`로 컴파일 오류 여부를 확인합니다.
