# 태스크 1-1: BuildTarget을 target.rs로 추출

## 1. 목표
`BuildTarget` 열거형과 기본 구현을 `src/core/model.rs`에서 새로운 파일 `src/core/target.rs`로 이동합니다.

## 2. 작업 내용
- `src/core/target.rs` 파일 생성.
- `src/core/model.rs`에서 `BuildTarget` 정의 복사.
- 필요한 라이브러리(`serde` 등) 임포트 추가.
- `src/core/model.rs`에서 기존 `BuildTarget` 정의 제거.

## 3. 성공 기준
- `src/core/target.rs` 파일이 존재하고 `BuildTarget` 정의를 포함함.
- (다음 태스크에서 임포트 수정 후) 프로젝트가 정상적으로 컴파일됨.
