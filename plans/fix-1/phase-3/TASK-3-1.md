# TASK-3-1: 마크다운 보존(Formatting Preservation) 단위 테스트 추가

## 목적
`MdPatcher`가 `description` 필드와 본문을 업데이트할 때, 원본 파일의 주석, 들여쓰기, 빈 줄 등이 훼손되지 않는지 엄격하게 검증합니다.

## 작업 내용

1. **MdPatcher 단위 테스트 확장 (`src/syncer/patcher.rs`)**
   - Frontmatter 내부에 복잡한 주석이 포함된 경우의 테스트 케이스 추가.
   - `description` 필드 앞뒤에 다른 필드가 밀집된 경우의 보존 테스트.
   - 마크다운 본문에 코드 블록(```)이나 HTML 주석이 포함된 경우의 패치 테스트.
   - 개행 문자가 `
`인 경우와 `
`인 경우의 일관성 확인.

## 검증 방법
- `cargo test syncer::patcher` 명령으로 모든 단위 테스트 통과 확인.
- `diff` 도구를 사용하여 수동으로 소스 파일의 변경 전후 포맷 비교.
