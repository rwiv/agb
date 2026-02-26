# TASK-1-1: update_description 반환 타입 변경 및 검증 로직 구현

## 목표
`MdPatcher::update_description`이 멀티라인 데이터를 감지하고 안전하게 에러를 반환하도록 수정합니다.

## 작업 내용
1. `src/syncer/patcher.rs`의 `update_description` 시그니처 변경:
   ```rust
   pub fn update_description(&mut self, new_desc: &str) -> anyhow::Result<()>
   ```
2. 검증 로직 추가:
   - `new_desc.contains('
')` 체크.
   - 원본 라인의 `ends_with('|')` 또는 `ends_with('>')` 체크.
   - 다음 줄이 공백(` `)으로 시작하는지 체크.
3. 기존 로직 성공 시 `Ok(())` 반환.

## 주의 사항
- `anyhow::Result`와 `anyhow::bail!`을 사용합니다.
- 기존의 정상적인 단일 라인 교체 기능이 깨지지 않아야 합니다.
