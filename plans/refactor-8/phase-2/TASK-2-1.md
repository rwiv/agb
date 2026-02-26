# 태스크 2-1: BuildTarget에 메타데이터 메서드 구현

## 1. 목표
타겟 전용 메타데이터 병합 로직을 `BuildTarget` 열거형 내부로 캡슐화합니다.

## 2. 작업 내용
- `reserved_key(&self) -> &'static str` 구현.
- `all_reserved_keys() -> &'static [&'static str]` 구현.
- `merge_metadata(&self, base: &mut Value, external: &Value)` 구현.
- `crate::core::constants`의 기존 상수들을 활용.

## 3. 성공 기준
- `BuildTarget`이 필요한 메서드들을 가짐.
- 로직이 Shallow merge 및 타겟별 오버라이드를 정확히 처리함.
