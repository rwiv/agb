# 리팩토링 계획: `load_resources` 함수 분해 및 구조화

## 1. 개요
`src/resource/loader.rs`의 `load_resources` 함수는 현재 파일 스캔 결과를 분류(Grouping)하고, 내용을 읽어 파싱(Parsing)하며, 최종 `Resource` 객체를 생성하는 모든 책임을 가지고 있습니다. 이를 기능별로 분리하여 가독성과 유지보수성을 향상시킵니다.

## 2. 목표
- `load_resources` 함수의 거대화(God Function) 방지
- 파일 분류 로직과 데이터 로드 로직의 명확한 분리
- 단위 테스트가 용이한 구조로 개선

## 3. 리팩토링 단계 (Phase 1)

### Phase 1: 함수 추출 및 파이프라인 구성
- **TASK-1-1**: 파일 분류 로직(`86-146행`)을 `group_files_by_resource` 함수로 추출
- **TASK-1-2**: 개별 리소스 생성 로직(`148-183행`)을 `parse_resource` 함수로 추출
- **TASK-1-3**: `load_resources` 함수를 추출된 함수들을 사용하는 파이프라인 형태로 재구성

## 4. 기대 효과
- 각 단계의 입력과 출력이 명확해짐 (`Vec<PathBuf>` -> `HashMap` -> `Vec<Resource>`)
- 새로운 리소스 타입이나 메타데이터 포맷 추가 시 변경 범위가 국소화됨
