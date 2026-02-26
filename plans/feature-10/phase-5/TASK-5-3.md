# TASK 5-3: Edge Case and Security Checks

## 개요 (Description)
`agb sync`가 비정상적이거나 위험한 상황에서 안전하게 동작하는지 검증하는 예외 처리 및 보안 테스트를 수행합니다.

## 수정 파일 (Files to Modify)
- `src/syncer/mod.rs` (예외 처리 추가)
- `tests/e2e_sync_test.rs` (예외 테스트 추가)

## 상세 지침 (Actionable Instructions)
1. `source_path`가 `Registry`에 등록된 소스 루트 디렉터리를 벗어나지 않는지 절대 경로 검증(Absolute Path Validation)을 수행합니다.
2. 타겟 파일이 손상되었거나 파싱 불가능한 경우, 전체 동기화 중단 대신 해당 파일만 건너뛰고 에러 로그를 기록하는지 테스트합니다.
3. 소스 파일이 쓰기 금지된 경우, 적절한 에러 메시지를 출력하는지 확인합니다.
4. `GEMINI.md` 등 메인 지침 파일에 대해 수정을 시도할 때 차단되는지 확인합니다.

## 검증 방법 (Verification)
- `cargo test --test e2e_sync_test`에서 예외 상황(Edge cases)을 시뮬레이션하여 검증합니다.
- 파일 권한, 잘못된 경로 설정 등을 고의적으로 유도하여 테스트합니다.
