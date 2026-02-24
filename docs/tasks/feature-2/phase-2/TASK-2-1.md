# TASK-2-1: YAML 확장자 스캔 지원

## 목표
- 플러그인 디렉터리 스캔 시 `.yaml` 및 `.yml` 파일을 인식하도록 수정.

## 작업 내용
1. `src/resource/loader.rs`의 `scan_plugins` 로직 확인.
2. 메타데이터 파일로 분류될 수 있는 확장자 목록에 `yaml`, `yml` 추가.

## 성공 기준
- 스캔된 파일 목록에 `.yaml` 및 `.yml` 파일이 포함됨.
