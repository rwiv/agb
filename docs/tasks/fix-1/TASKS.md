# Master Task List: agb (Agents Builder)

각 테스크는 독립적인 `docs/tasks/fix-1/phase-X/TASK-X-Y.md` 문서에 상세 가이드를 가집니다.

## Phase 1: 소스 및 출력 경로 분리 (Source-Output Decoupling)
*   **Task 1.1: `agb.yaml` 모델 확장 및 `source` 필드 추가**
    *   *성공 기준:* `agb.yaml`에 절대 경로 `source` 필드를 추가하고, 이를 Rust `Config` 구조체로 정확히 파싱한다.
*   **Task 1.2: 소스 로딩 및 파일 출력 경로 분리 로직 구현**
    *   *성공 기준:* 리소스 스캔 및 로딩은 `source` 경로에서 수행하고, 파일 생성(`Emitter`)은 `agb.yaml`이 위치한 디렉터리에서 수행한다.
*   **Task 1.3: 경로 분리 아키텍처 통합 및 최종 검증**
    *   *성공 기준:* 소스 디렉터리와 빌드 결과물 디렉터리가 다른 환경에서 `agb build` 명령이 성공적으로 작동함을 테스트로 증명한다.
