# Core 모듈

## 개요
`core` 모듈은 `agb` 프로젝트의 의존성 구조에서 최하단에 위치하며, 시스템 전역에서 사용하는 공용 데이터 모델과 타입을 정의합니다. 다른 모든 모듈(`loader`, `builder`, `transformer`)은 이 모듈에 정의된 타입을 기초로 동작합니다.

## 주요 구성 요소

### 1. 데이터 모델 (`model.rs`)
에이전트 리소스의 핵심 구조와 상태를 정의합니다.

- **BuildTarget**: 빌드 대상 플랫폼 규격(`gemini-cli`, `claude-code`, `opencode`)을 정의하는 열거형입니다.
- **Resource**: `Command`, `Agent`, `Skill` 타입을 지원하는 핵심 열거형입니다. 각 타입은 `ResourceData`를 포함합니다.
- **ResourceData**: 
  - `name`: 리소스 식별 이름.
  - `plugin`: 리소스가 소속된 플러그인 이름.
  - `content`: 마크다운 본문 내용.
  - `metadata`: YAML/Frontmatter에서 파싱된 설정 값 (`serde_json::Value`).
- **ResourceKey**: 리소스를 고유하게 식별하기 위한 키 구조체 (`plugin`, `type`, `name`).
- **ResourcePaths**: 리소스를 구성하는 파일 경로들의 집합 (`md` 파일 경로, `metadata` 파일 경로).
- **TransformedFile**: 변환기(Transformer)를 거쳐 최종적으로 파일 시스템에 출력될 경로와 내용을 담는 구조체입니다.

## 설계 원칙
- **무의존성**: `core` 모듈은 프로젝트 내의 다른 모듈을 참조하지 않습니다.
- **데이터 중심**: 비즈니스 로직보다는 데이터의 구조와 직렬화/역직렬화 규격을 정의하는 데 집중합니다.
