# TASK-2-2: 메타데이터 파싱 및 충돌 감지

## 목표
- YAML 메타데이터 파싱 기능 구현 및 중복 포맷 발견 시 에러 발생.

## 작업 내용
1. `src/resource/loader.rs`의 `load_resources` 함수 수정.
2. 리소스 그룹화 단계에서 메타데이터 경로가 중복 설정되는지 검사 (JSON/YAML 동시 존재).
3. 파일 확장자에 따라 `serde_json` 혹은 `serde_yaml`을 사용하여 `ResourceData` 생성.
4. 충돌 발생 시 구체적인 에러 메시지(`bail!`) 반환.

## 성공 기준
- `.yaml` 메타데이터가 정상 로드됨.
- 동일 리소스에 `.json`과 `.yaml`이 공존할 경우 빌드가 실패함.
