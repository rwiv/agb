# TASK-1-2: MdPatcher의 description 업데이트 로직 강화

## 목적
메타데이터 중 `description` 필드만 동기화하는 기존 정책을 유지하되, 현재의 취약한 정규표현식 로직을 개선하여 원본 마크다운의 다른 필드나 포맷을 훼손하지 않고 안전하게 값을 교체하도록 합니다.

## 작업 내용

1. **MdPatcher 수정 (`src/syncer/patcher.rs`)**
   - `update_description` 메서드가 `description` 키를 찾을 때, 주변 공백이나 인용구(`"`, `'`) 처리를 보다 유연하게 하도록 정규표현식 개선.
   - 값 교체 시 기존의 인용 형식을 최대한 보존하거나, 필요 시 표준 YAML 인용을 적용.
   - Frontmatter 내부의 필드 순서와 사용자 주석을 100% 보존함을 보장.

2. **유틸리티 활용**
   - `src/utils/yaml.rs` 등에 정의된 로직이 있다면 이를 활용하여 Frontmatter 영역을 보다 안전하게 식별.

## 검증 방법
- `patcher.rs` 내의 유닛 테스트에 다음 케이스 추가:
  - `description` 필드가 여러 줄인 경우.
  - `description` 필드 뒤에 다른 필드가 있는 경우.
  - 마크다운 본문에 `---`와 유사한 문자열이 포함된 경우.
