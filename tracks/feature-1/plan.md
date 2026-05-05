# Plan: Codex OpenAI 정책 파일 생성

## Phase 1: Codex build 산출물 생성

### Task 1.1: Command·Skill OpenAI 정책 파일 생성

- [x] `agents/openai.yaml` 상대 경로 상수 추가 (`src/core/constants.rs`)
- [x] Codex 정책 파일 내용과 `disable-model-invocation` 판정 helper 추가 (`src/transformer/codex.rs`)
- [x] `CodexTransformer::post_transform()`에서 기존 agent registry 생성과 정책 파일 생성을 함께 처리 (`src/transformer/codex.rs`)
- [x] Command는 metadata boolean `true`일 때 정책 파일을 항상 생성 (`src/transformer/codex.rs`)
- [x] Skill은 metadata boolean `true`이고 source extra `agents/openai.yaml`이 없을 때만 정책 파일 생성 (`src/transformer/codex.rs`)
- [x] 정책 파일 생성 단위 테스트 추가 (`src/transformer/codex.rs`)
- [x] Codex 변환 명세 갱신 (`specs/spec.md`)
- [x] Transformer 모듈 문서 갱신 (`src/transformer/README.md`)

### Task 1.2: Sync 제외 처리 및 E2E 검증

- [x] Codex 빌드 대상 Skill의 source `agents/openai.yaml`이 exclude에 걸리면 fail-fast 검증 추가 (`src/app/context.rs`)
- [x] `Transformer` trait에 생성 extra ignore hook 추가 (`src/transformer/mod.rs`)
- [x] `CodexTransformer`에서 생성된 Skill 정책 파일만 ignore path로 반환 (`src/transformer/codex.rs`)
- [x] `ExtraSyncer`에 ignore path 지원 추가 (`src/syncer/extra.rs`)
- [x] `Syncer::sync_resource()`에서 Skill extra sync 시 transformer ignore path 전달 (`src/syncer/mod.rs`)
- [x] exclude에 걸린 source policy 파일 검증 단위 테스트 추가 (`src/app/context.rs`)
- [x] ignore path Add/Update/Delete 단위 테스트 추가 (`src/syncer/extra.rs`)
- [x] Codex build, 최종 메타데이터 병합, source extra 복사, build --clean, sync update/delete E2E 테스트 추가 (`tests/e2e_codex_openai_policy_test.rs`)
- [x] Syncer 모듈 문서 갱신 (`src/syncer/README.md`)
- [x] Codex sync 명세 갱신 (`specs/spec.md`)
- [x] `cargo test`와 `cargo clippy --all-targets -- -D warnings` 실행
