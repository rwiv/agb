# TASK 1-3: Hash-based Comparison for Extra Files

## 개요 (Description)
`Syncer`가 스킬 내의 `ExtraFiles` 변경을 감지하기 위해 SHA-256 해시 비교 기능을 준비합니다.

## 수정 파일 (Files to Modify)
- `Cargo.toml` (필요 시 `sha2` 디펜던시 추가)
- `src/utils/fs.rs` (해시 계산 유틸리티 추가)

## 상세 지침 (Actionable Instructions)
1. `sha2` 크레이트를 프로젝트에 추가합니다. (이미 존재하거나 `anyhow` 외의 표준 라이브러리를 통해 간단히 구현 가능하다면 생략 가능하지만, `sha2` 추천)
2. `src/utils/fs.rs`에 파일의 해시를 계산하는 `calculate_hash<P: AsRef<Path>>(path: P) -> Result<String>` 함수를 추가합니다.
3. 이 유틸리티 함수에 대한 유닛 테스트를 작성하여, 파일 내용이 1비트만 달라도 해시값이 달라지는지 검증합니다.

## 검증 방법 (Verification)
- `cargo test utils::fs::tests`를 실행하여 해시 계산 기능의 정확성을 확인합니다.
- 바이너리 파일과 텍스트 파일 모두에 대해 테스트합니다.
