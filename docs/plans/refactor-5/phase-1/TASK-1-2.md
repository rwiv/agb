# Task 1.2: 검증 및 정리

## 1. Objective (목표)

- `Task 1.1`에서 수행한 리팩토링의 최종 무결성을 검증합니다.
- `cargo test`를 통해 모든 기능이 정상 작동하는지 확인합니다.
- 순환 의존성이 완전히 제거되었는지 점검합니다.

## 2. Context & Files (작업 범위)

- **검토 전용:**
  - `src/resource/mod.rs` (`BuildTarget` 노출 여부)
  - `src/transformer/mod.rs` (의존성 단방향 확인)
- **수정할 파일:**
  - `tests/` 내의 테스트 파일 (임포트 오류 발생 시 수정)

## 3. Instructions (세부 지침)

### Step 1: `cargo test` 실행

전체 테스트를 실행하여 컴파일 오류와 런타임 오류가 없는지 확인하세요.

```bash
cargo test
```

- 특히 `src/transformer/mod.rs`의 `tests` 모듈 내 `test_transformer_factory_filenames` 테스트가 여전히 정상 작동하는지 확인하세요.

### Step 2: 임포트 정리 및 최적화

`BuildTarget` 이동으로 인해 발생할 수 있는 불필요한 임포트 구문을 정리하세요.
- `src/builder/config.rs`에서 `BuildTarget`을 더 이상 직접 정의하지 않으므로, 이를 참조하던 다른 파일들이 있다면 `resource::BuildTarget`을 사용하도록 일관성 있게 수정합니다.

### Step 3: 순환 의존성 여부 최종 점검

`transformer` 모듈에서 `builder` 모듈을 참조하는 부분이 없는지 확인하세요.

- **Check Command:** `grep -r "crate::builder" src/transformer`
- 결과가 없거나, `Transformer` 트레이트나 팩토리와 무관한 부분이어야 합니다. (이 경우에는 아예 없어야 함)

## 4. Constraints (제약 사항 및 금지 행동)

- 테스트를 위해 임의로 로직을 변경하지 마세요.
- `cargo test`가 하나라도 실패하면 원인을 파악하여 `Task 1.1`의 수정 사항을 보완해야 합니다.

## 5. Acceptance Criteria (검증 체크리스트)

1. `cargo test`가 모든 모듈에서 성공하는가?
2. `src/transformer/mod.rs`의 `tests` 구문이 정상 작동하는가?
3. `builder`와 `transformer` 간의 양방향 의존성이 완전히 해결되어 단방향(`builder` -> `transformer` -> `resource`) 구조가 되었는가?
