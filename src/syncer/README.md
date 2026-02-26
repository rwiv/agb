# Syncer 모듈

`syncer` 모듈은 프로젝트 환경(Target)의 변경사항을 원본 소스(Source)로 역전파(Back-propagation)하는 핵심 엔진을 담당합니다. `app` 모듈에 의해 호출되어 개별 리소스의 동기화를 수행합니다.

## 주요 역할

1. **상태 비교 (Diff)**: 타겟 결과물과 원본 소스 사이의 메타데이터(`description`), 본문(`content`), 추가 파일(`extras`)의 차이점을 감지합니다.
2. **부분적 업데이트 (Partial Update)**: 원본 마크다운 파일의 구조(주석, 들여쓰기, 필드 순서 등)를 최대한 유지하면서 특정 필드(`description`)만 교체합니다.
3. **디렉터리 동기화**: 스킬 디렉터리 내의 추가 파일들을 해시 기반으로 비교하여 추가, 삭제, 수정을 수행합니다.

## 주요 구성 요소

### 1. Syncer (`mod.rs`)
전체 동기화 프로세스를 조율하는 실행기입니다.
- **`sync_resource` (Public API)**: 단일 리소스에 대해 타겟 파일을 읽고, `Transformer::detransform`을 통해 모델로 복원한 뒤, `MdPatcher` 및 `DirectorySyncer`를 사용하여 소스에 반영합니다.

### 2. MdPatcher (`patcher.rs`)
마크다운 파일의 구조와 포맷을 최대한 보존하면서 특정 부분만 수정하는 역할을 수행합니다.
- **부분적 업데이트 (Partial Update)**: 정규표현식을 사용하여 YAML Frontmatter의 `description` 필드만 교체하거나, 본문(`Body`) 영역을 전체 교체합니다.
- **안전성**: 멀티라인 `description`이나 복잡한 YAML 마커가 소스에서 감지될 경우, 원본 파손을 방지하기 위해 에러를 발생시킵니다.

### 3. SyncPlanner (`planner.rs`)
두 디렉터리(Source와 Target)를 비교하여 수행해야 할 작업 목록(`SyncAction`)을 생성합니다.
- **해시 기반 비교**: 파일의 SHA-256 해시를 비교하여 변경 여부를 판단합니다.
- **예외 처리**: `SKILL.md`와 같은 핵심 파일은 플래너 단계에서 제외하여 개별 로직에서 안전하게 처리될 수 있도록 합니다.

### 4. DirectorySyncer (`directory.rs`)
`SyncPlanner`가 수립한 계획(`SyncAction`)에 따라 실제 파일 시스템 작업을 수행합니다.
- 스킬 리소스의 `extras` 파일들을 동기화하는 데 주로 사용됩니다.

## 동기화 흐름

1. `app::App::sync`에서 `Syncer` 인스턴스를 생성합니다.
2. 빌드 대상 리소스들을 순회하며 `Syncer::sync_resource`를 호출합니다.
3. `sync_resource` 내부 동작:
    - 타겟 경로에서 파일을 읽어 `Transformer::detransform`으로 데이터 복원.
    - `MdPatcher`를 통해 소스 마크다운의 `description`과 본문 업데이트.
    - 리소스가 `Skill` 타입인 경우, `DirectorySyncer`를 호출하여 하위 디렉터리 파일들을 동기화.

## 설계 원칙
- **원본 보존 (Preservation)**: 소스 파일의 주석이나 포맷을 최대한 유지하기 위해 전체 파일을 다시 쓰지 않고 부분적 패치를 우선합니다.
- **무결성 (Integrity)**: 해시 비교를 통해 불필요한 파일 쓰기를 방지하고, 모호한 변경 사항에 대해서는 실패(Fail-fast)를 선택합니다.
