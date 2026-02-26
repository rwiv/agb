# TASK-1-1: Resource Loader 로직 수정

## 목표
`src/resource/loader.rs`에서 스킬 리소스를 로드할 때, 메타데이터 파일명을 고정값(`METADATA`)이 아닌 스킬 디렉터리 이름(`skill_name`)을 사용하도록 변경합니다.

## 작업 상세

1. **`load_resources` 함수 수정**:
   - `r_type == "skills"` 분기 내에서 `file_name`을 체크할 때, `skill_name`과 일치하는지 확인하도록 로직 변경.
   - 예: `skills/my_skill/` 폴더 내에서 `my_skill.json`, `my_skill.yaml`, `my_skill.yml`을 메타데이터로 인식.

```rust
// 수정 전
if file_name == "METADATA.json" || file_name == "METADATA.yaml" || file_name == "METADATA.yml" { ... }

// 수정 후 (개념 코드)
let is_metadata = (file_name == format!("{}.json", skill_name)) 
               || (file_name == format!("{}.yaml", skill_name))
               || (file_name == format!("{}.yml", skill_name));
if is_metadata { ... }
```

2. **마크다운 파일 매칭 규칙 유지**:
   - `*.md` 파일은 여전히 스킬의 컨텐츠로 유지하되, 관례상 `SKILL.md` 또는 `README.md`를 권장하는 주석 추가.

## 검증 계획
- `cargo test resource::loader`를 실행하여 기존 테스트가 실패하는 것을 확인 (파일명이 바뀌었으므로 당연한 결과).
- 이후 TASK-1-3에서 테스트 코드를 수정하여 최종 검증.
