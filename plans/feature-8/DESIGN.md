# Design Doc: Remove JSON Metadata Support

## 1. 개요 (Overview)
현재 `agb`는 메타데이터 파일로 JSON(`.json`)과 YAML(`.yaml`, `.yml`)을 모두 지원합니다. 하지만 중복 관리의 복잡성을 줄이고 설정 파일의 일관성을 높이기 위해 JSON 지원을 완전히 제거하고 YAML 포맷으로 단일화합니다.

## 2. 목표 (Goals)
- 메타데이터 파일 탐색 및 파싱 로직에서 JSON 관련 코드 완전 제거.
- 프로젝트 내 모든 테스트 픽스처 및 유닛 테스트에서 JSON 파일을 YAML로 전환.
- 중복 메타데이터(yaml/yml 동시 존재) 검증 로직을 기존 코드를 활용해 유지.

## 3. 상세 설계 (Detailed Design)

### 3.1 소스 코드 수정
- **`src/resource/loader/resolver.rs`**: 
    - `is_metadata_extension` 함수에서 `"json"` 확장자 제거.
- **`src/resource/loader/parser.rs`**: 
    - `parse_metadata` 함수 내의 JSON 파싱 매치 암(match arm) 제거.
- **`src/transformer/gemini.rs`**:
    - Gemini 변환 시 여전히 내부적으로는 `serde_json::Value`를 사용하되, 입력 소스는 YAML에서 온 데이터임을 전제로 함. (기존 로직 유지 가능)

### 3.2 테스트 및 픽스처 수정
- **`tests/fixtures/`**:
    - 모든 `.json` 파일을 `.yaml`로 변환.
- **유닛 테스트**:
    - `loader/mod.rs`, `loader/parser.rs`, `loader/resolver.rs` 등에서 JSON을 생성하고 검증하는 테스트 케이스를 YAML로 수정하거나 삭제.

### 3.3 에러 처리 및 중복 검증
- `ResourcePathResolver::validate_metadata_uniqueness` 로직은 이미 존재하므로, 인식 대상 확장자에서 `json`이 빠지면 자동으로 `yaml`과 `yml`이 충돌할 때 에러를 발생시킴.

## 4. 기대 효과 (Expected Impact)
- 메타데이터 포맷 단일화로 인한 유지보수성 향상.
- 빌드 결과물의 예측 가능성 증대.

## 5. 단계별 계획 (Phases)
1. **Phase 1**: 소스 코드에서 JSON 인식 및 파싱 로직 제거.
2. **Phase 2**: 테스트용 픽스처 및 유닛 테스트 코드 수정.
3. **Phase 3**: 문서(README.md, SPEC.md 등) 최신화.
