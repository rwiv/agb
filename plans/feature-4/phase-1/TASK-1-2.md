# TASK-1-2: 시스템 문서 업데이트

## 목표
프로젝트의 전반적인 기술 문서와 `README.md`에서 스킬 메타데이터에 대한 설명을 실제 구현과 일치하도록 업데이트합니다.

## 작업 상세

1. **`README.md` 수정**:
   - `skills/` 디렉터리 구조 예시에서 `METADATA.json`을 `[skill_name].json`으로 변경.
2. **`specs/SYSTEM_SPEC.md` 수정**:
   - `2.2 소스 디렉터리 구조` 섹션에서 `METADATA.json/yaml/yml`을 `[skill_name].json/yaml/yml`로 수정.
3. **`src/resource/README.md` 수정**:
   - `skills/` 리소스 구성 상세 섹션에서 `METADATA.yaml` 관련 설명을 `[skill_name].yaml`로 변경.

## 검증 계획
- 문서 내 모든 `METADATA` 언급이 적절하게 수정되었는지 수동 검토.
- `grep`을 통해 더 이상 스킬 설정 파일로 `METADATA`가 언급되지 않는지 확인.
