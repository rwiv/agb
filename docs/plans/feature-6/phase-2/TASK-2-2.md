# Task 2-2: Implement Extraction & Merge in parse_resource

## 1. 개요
`ResourceParser`의 핵심 메서드인 `parse_resource`에 이전 단계에서 구현한 추출 및 병합 로직을 통합합니다.

## 2. 상세 요구사항
- **변경 지점**: `src/resource/loader/parser.rs`의 `parse_resource` 함수.
- **데이터 흐름 수정**:
    1. `md` 파일이 존재할 경우 내용을 읽고 `extract_frontmatter`를 호출하여 `fm_metadata`와 `pure_content` 획득.
    2. 외부 메타데이터 파일(`metadata`)이 존재할 경우 기존처럼 `parse_metadata`로 로드하여 `ext_metadata` 획득.
    3. `merge_metadata(&mut fm_metadata, &ext_metadata, &self.target)` 호출 (Task 1-2에서 구현한 로직).
    4. 생성된 `ResourceData` 객체의 `content` 필드에는 `pure_content`를, `metadata` 필드에는 병합된 결과를 할당.

## 3. 리소스 이름(`name`) 결정 로직 유지
- 현재 코드는 파일명을 기본 `name`으로 사용함.
- 병합된 메타데이터에 `"name"` 키가 명시되어 있다면 해당 값을 `ResourceData.name`에 할당하도록 로직을 보강하여 사용자의 명시적 설정을 존중해야 함.

## 4. 검증 기준
- 빌드 결과물(`commands/*.toml` 등)의 `prompt` 필드에 `---` 구분자가 절대 포함되지 않아야 함.
- 메타데이터 오버라이트가 적용된 상태로 파일이 생성되어야 함.
