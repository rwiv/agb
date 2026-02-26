# 태스크 3-1: ResourceParser 리팩토링

## 1. 목표
`ResourceParser`가 타겟의 구체적인 구현 대신 `BuildTarget`이 제공하는 추상화된 메서드를 사용하도록 코드를 간소화합니다.

## 2. 작업 내용
- `src/loader/parser.rs`의 `ResourceParser::merge_metadata` 메서드를 수정하여 `self.target.merge_metadata(...)`를 호출하도록 변경.
- `parse_resource` 메서드 내에서 타겟 전용 로직이 남아있는지 확인하고 제거.

## 3. 성공 기준
- `ResourceParser`의 코드가 더 간결해지고 추상화 수준이 높아짐.
- 기존의 기능(메타데이터 병합 및 이름 추출 등)이 동일하게 작동함.
