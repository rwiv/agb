# TASK 5-2: Update Module Documentation (`src/builder/README.md`)

## 개요 (Description)
`builder` 모듈 내에 새롭게 추가된 의존성 검사(Dependency Check) 기능에 대한 설명을 보충합니다.

## 수정 파일 (Files to Modify)
- `src/builder/README.md`

## 상세 지침 (Actionable Instructions)
1. **`src/builder/README.md`**:
    - `builder` 모듈의 구성 요소 목록에 `DependencyChecker`를 추가합니다.
    - 빌드 프로세스 설명 섹션에서 `Emitter` 호출 전 리소스 간의 정적 의존성 검증이 수행됨을 명시합니다.

## 검증 방법 (Verification)
- `src/builder/README.md` 문서가 실제 소스 코드(`src/builder/dependency.rs`, `src/builder/mod.rs`)와 일치하는지 확인합니다.
