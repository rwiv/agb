# TASK 3-4: Implement `Syncer` Orchestrator

## 개요 (Description)
`agb sync` 기능을 총괄하는 `syncer::Syncer` 객체와 전체 동기화 파이프라인을 구현합니다.

## 수정 파일 (Files to Modify)
- `src/syncer/mod.rs` (신규 파일)

## 상세 지침 (Actionable Instructions)
1. `syncer` 모듈을 생성합니다.
2. `Syncer` 구조체와 `new(config: &Config)` 메서드를 정의합니다.
3. `run(&self) -> Result<()>` 메서드를 구현하여 전체 동기화 과정을 실행합니다.
    - `loader::ResourceLoader`를 사용해 소스 기준의 `Registry`를 구축합니다.
    - 타겟 디렉터리(`dest`)를 스캔하여 등록된 리소스들과 대조합니다.
    - 각 리소스에 대해 `Transformer::detransform`을 호출하여 `ResourceData`를 복원합니다.
    - 소스 파일의 `description`과 `content`를 타겟의 내용과 비교하여 변경사항이 있다면 `Surgical Update`를 수행합니다.
    - 스킬 타입의 경우 `sync_skill_dir`을 호출하여 추가/삭제/수정을 동기화합니다.
4. `GEMINI.md`, `CLAUDE.md`, `OPENCODE.md` 등의 메인 지침 파일은 명시적으로 건너뛰도록 예외 처리를 추가합니다.

## 검증 방법 (Verification)
- `syncer::mod::tests`를 작성하여, `Syncer`의 오케스트레이션 로직이 정상적으로 작동하는지 검증합니다.
- 전체 프로세스 실행 시 로그가 명확하게 출력되는지 확인합니다.
