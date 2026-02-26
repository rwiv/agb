# TASK 1-1: `src/transformers` 디렉토리 이름을 `src/transformer`로 변경

## 1. 개요
현재 `src/transformers` 모듈의 디렉토리 이름을 단수형인 `src/transformer`로 변경합니다. 이 변경은 전체 리팩토링의 시작점이며, 다른 소스 코드에서의 모듈 임포트 구문을 연쇄적으로 업데이트해야 합니다.

## 2. 세부 작업 지침
- `src/transformers/` 디렉토리를 `src/transformer/`로 이동시킵니다.
- `git mv` 명령을 사용하여 변경 이력을 유지합니다.
- 하위 파일(base.rs, factory.rs, mod.rs, README.md, providers/*)의 내부 구조는 변경하지 않습니다.

## 3. 성공 기준
- `src/transformer/` 디렉토리에 기존의 모든 파일이 정확히 위치해야 합니다.
- `src/transformers/` 디렉토리는 더 이상 존재하지 않아야 합니다.
