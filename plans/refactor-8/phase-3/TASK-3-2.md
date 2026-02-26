# 태스크 3-2: parser.rs 내 타겟 전용 상수 제거

## 1. 목표
`ResourceParser`에서 더 이상 사용하지 않는 타겟 관련 상수 임포트와 의존성을 제거합니다.

## 2. 작업 내용
- `src/loader/parser.rs` 상단의 `TARGET_GEMINI`, `TARGET_CLAUDE`, `TARGET_OPENCODE` 임포트 제거.
- `match` 문 등을 통해 타겟을 분기 처리하던 로직이 모두 제거되었는지 최종 확인.

## 3. 성공 기준
- `parser.rs` 파일 내에 특정 타겟(Gemini, Claude 등)을 지칭하는 문자열이나 상수가 존재하지 않음.
