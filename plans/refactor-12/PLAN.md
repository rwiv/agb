# PLAN: Syncer 모듈 리팩토링 및 고도화

## 개요
`Syncer` 모듈의 복잡도를 낮추고, 일반화된 디렉토리 동기화 기능을 도입하며, 표준 로깅을 적용합니다.

## 단계별 계획

### Phase 1: Resource 모델 다형성 강화 (Helper Methods 추가)
- [ ] `src/core/model.rs`: `Resource` Enum에 `main_source_path()`, `metadata()`, `content()` 메서드 추가
- [ ] `src/syncer/mod.rs`: `sync_resource` 함수 내의 `match` 분기 로직을 헬퍼 메서드 사용으로 교체
- [ ] 관련 유닛 테스트 확인 및 수정

### Phase 2: DirectorySyncer 도입 (SkillSyncer 일반화)
- [ ] `src/syncer/skill.rs`를 `src/syncer/directory.rs`로 이름 변경
- [ ] `SkillSyncer` 구조체를 `DirectorySyncer`로 리네임 및 관련 메서드 일반화
- [ ] `src/syncer/mod.rs`: `DirectorySyncer` 호출부 반영

### Phase 3: 표준 로깅 적용 (println! 제거)
- [ ] `Cargo.toml`: `log`, `env_logger` 의존성 추가
- [ ] `src/main.rs`: `env_logger::init()` 초기화 추가
- [ ] `src/syncer/` 내의 모든 `println!`을 `log::info!`, `log::debug!` 등으로 교체
- [ ] 기타 모듈의 `println!` 전수 조사 및 교체 (범위 내)

## 성공 기준
1. `Syncer::sync_resource`의 복잡도가 현저히 감소 (분기 처리 최소화).
2. `DirectorySyncer`가 특정 리소스 타입에 의존하지 않고 독립적으로 동작.
3. `RUST_LOG=info agb sync` 실행 시 필요한 정보만 로그로 출력됨.
4. 모든 E2E 동기화 테스트 통과.
