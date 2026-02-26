# 계획: BuildTarget 분리 및 결합도 낮추기

## 1. 개요
이 계획은 `BuildTarget`을 독립된 모듈로 추출하고, `ResourceParser`가 특정 타겟의 세부 사항에 의존하지 않도록 결합도를 낮추는 과정을 설명합니다.

## 2. 단계 및 태스크

### 1단계: 핵심 리팩토링 (추출)
- [x] **TASK 1-1**: `src/core/target.rs` 파일을 생성하고 `src/core/model.rs`에서 `BuildTarget` 열거형을 이동.
- [x] **TASK 1-2**: `src/core/mod.rs`를 수정하여 새로운 `target` 모듈을 내보냄(export).
- [x] **TASK 1-3**: 코드베이스 전체의 임포트(import) 경로를 `crate::core::target::BuildTarget`으로 업데이트.

### 2단계: 로직 캡슐화
- [x] **TASK 2-1**: `src/core/target.rs`의 `BuildTarget`에 `reserved_key`, `all_reserved_keys`, `merge_metadata` 메서드 구현.
- [x] **TASK 2-2**: `src/loader/parser.rs`의 메타데이터 병합 로직을 `BuildTarget::merge_metadata`로 이동.

### 3단계: 결합도 제거 및 정리
- [x] **TASK 3-1**: `ResourceParser`가 새로운 `BuildTarget::merge_metadata` 메서드를 사용하도록 수정.
- [x] **TASK 3-2**: `src/loader/parser.rs`에서 타겟 전용 상수(`TARGET_GEMINI` 등) 사용을 제거.
- [x] **TASK 3-3**: 전체 테스트(`cargo test`)를 통한 검증.

## 3. 성공 기준
- [x] `BuildTarget`이 독립된 파일 `src/core/target.rs`에 존재함.
- [x] `ResourceParser` 내부에 타겟 전용 문자열 리터럴이나 `match` 문이 제거됨.
- [x] 기존의 모든 테스트가 통과함.
- [x] `BuildTarget::merge_metadata`에 대한 새로운 단위 테스트가 추가됨.
