# 리팩토링 계획: 리소스 로더 모듈화 및 팩토리 객체화

리소스 로딩 로직의 복잡도를 낮추기 위해 `ResourceLoader`를 별도 모듈로 분리하고, 기능을 객체 단위로 세분화합니다. 또한 `TransformerFactory`를 객체 구조로 리팩토링하여 일관성을 확보합니다.

## 목표
- [ ] `src/resource/loader.rs`를 `src/resource/loader/` 디렉터리 기반의 서브 모듈 구조로 분리.
- [ ] 로딩 로직을 `FileFilter`, `PathResolver`, `MetadataParser` 객체로 분해하여 책임 분리.
- [ ] 각 서브 모듈의 책임에 맞게 기존 테스트 코드를 재배치하여 유닛 테스트 완결성 확보.
- [ ] `TransformerFactory`를 함수 형태에서 구조체 기반으로 리팩토링.
- [ ] 기존 테스트 코드를 유지하면서 객체 지향적인 인터페이스 제공.

## 단계별 계획

### Phase 1: Resource Loader 모듈화 및 기능 분해
로딩 파이프라인을 각 책임별 객체로 분리하고, 관련 유닛 테스트를 각 서브 모듈로 이동합니다.
- **TASK-1-1**: `ResourceLoader` 기반 구조 및 `FileFilter` 구현 (필터링 테스트 이동)
- **TASK-1-2**: `MetadataParser` 구현 및 통합 (파싱 테스트 이동)
- **TASK-1-3**: `ResourcePathResolver` 구현 (경로 분석 및 충돌 테스트 이동)
- **TASK-1-4**: `ResourceLoader` 최종 통합 및 `src/resource/mod.rs` 업데이트 (통합 테스트 구성)

### Phase 2: Transformer Factory 리팩토링
단순 함수로 정의된 팩토리를 구조체 기반으로 변경합니다.
- **TASK-2-1**: `TransformerFactory` 구조체 도입 및 인터페이스 변경

### Phase 3: 통합 및 검증
변경된 객체들을 실제 빌드 흐름에 적용하고 검증합니다.
- **TASK-3-1**: `src/builder/core.rs` 내 로더 및 팩토리 호출부 수정
- **TASK-3-2**: 전체 유닛 테스트 및 E2E 테스트 실행을 통한 회귀 검사
