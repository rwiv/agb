# TASK 4-3: Complex Dependency Scenarios

## 개요 (Description)
여러 플러그인과 다양한 리소스 타입이 얽힌 복잡한 의존성 관계에서의 동작을 검증합니다.

## 수정 파일 (Files to Modify)
- `tests/e2e_dependency_test.rs`

## 상세 지침 (Actionable Instructions)
1. 다중 의존성 시나리오 추가: 하나의 리소스가 여러 타입의 리소스를 의존하는 경우.
2. 순환 참조 시나리오(A->B, B->A)에서도 빌드 대상에 포함되어 있다면 정상 동작하는지 확인합니다.

## 검증 방법 (Verification)
- 추가된 복합 시나리오 테스트 케이스들이 모두 통과하는지 확인합니다.
