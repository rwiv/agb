# TASK 3-1: 표준 로깅 적용 (println! 제거)

## 목적
표준 `log` 크레이트를 도입하여 CLI 출력의 일관성을 확보하고, 디버깅을 용이하게 합니다.

## 작업 내용
- `Cargo.toml`에 `log`, `env_logger` 라이브러리 추가.
- `src/main.rs`의 `main` 함수 초입부에 `env_logger::init()` 호출 추가.
- `src/syncer/mod.rs` 및 `src/syncer/directory.rs` (또는 `skill.rs`) 내의 모든 `println!`을 적절한 로그 레벨(`log::info!`, `log::debug!`, `log::warn!`)로 교체.
- 기타 `src/` 디렉토리 내의 잔여 `println!` 전수 조사 및 로그로 변환.

## 검증 방법
- `RUST_LOG=info agb sync` 실행 시 정상적인 메시지가 출력되는지 확인.
- `cargo test` 실행하여 로깅 라이브러리 추가로 인한 영향이 없는지 확인.
- `RUST_LOG=debug agb sync` 실행 시 상세 로그 출력이 되는지 확인.
