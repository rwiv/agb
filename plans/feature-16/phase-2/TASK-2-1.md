# Task 2.1: `spec.md` 및 `design.md` 갱신

## 개요
코드 레벨의 동작 변경 사항(프론트매터 순서 보존)을 프로젝트의 기술 명세서(Specifications) 및 시스템 설계(System Design) 문서에 반영하여 아키텍처 문서화 상태를 최신으로 유지합니다.

## 작업 내용
1. **`specs/spec.md`**:
    * '5. 동기화 규격 (Sync Specifications)' 섹션 중 'MdPatcher'의 YAML 파싱 설명 부분을 수정하여, "순서는 `serde_json`의 `preserve_order` 피처를 통해 보장된다"는 내용을 추가한다.
2. **`specs/design.md`**:
    * '3.3 동기화 패치 알고리즘 (MdPatcher)' 부분에 정규표현식이 아닌 객체 파싱 후 재직렬화 방식을 유지하되 `preserve_order`를 통해 순서를 보존한다는 기술적 결정을 기록한다.

## 갱신 대상 문서 파일
- `specs/spec.md`
- `specs/design.md`

## 성공 기준
* 위 두 문서에 `preserve_order` 피처를 활용하여 프론트매터 순서를 보장한다는 내용이 명확하게 기술되어 있어야 한다.
