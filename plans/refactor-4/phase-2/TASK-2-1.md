# TASK-2-1: TransformerFactory 구조체 도입 및 인터페이스 변경

## 목표
- 단순 함수 형태인 `get_transformer`를 `TransformerFactory` 구조체의 메서드로 변경하여 일관성을 확보하고 확장성을 열어둠.

## 상세 작업 내용

### 1. `TransformerFactory` 구현 (`src/transformer/factory.rs`)
- 인터페이스 설계:
  ```rust
  pub struct TransformerFactory;

  impl TransformerFactory {
      pub fn create(target: &BuildTarget) -> Box<dyn Transformer>;
  }
  ```
- 기존 `get_transformer` 함수를 `create` 메서드로 변경.

### 2. 하위 호환성 (선택 사항)
- 기존 `get_transformer` 함수를 유지하면서 내부에서 `TransformerFactory::create`를 호출하도록 하여 점진적 전환 지원.

## 검증 계획
- 기존 `test_transformer_factory_filenames` 테스트가 변경된 인터페이스에서도 정상 작동하는지 확인.
