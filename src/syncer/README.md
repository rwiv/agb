# Syncer 모듈

`syncer` 모듈은 타겟 에이전트 환경의 변경사항을 원본 소스로 동기화하는 핵심 엔진을 담당합니다.

## 주요 역할

1. **상태 비교 (Diff)**: 타겟 결과물과 원본 소스 사이의 메타데이터(`description`), 본문(`content`), 추가 파일(`extras`)의 차이점을 감지합니다.
2. **수술적 업데이트 (Surgical Update)**: 원본 마크다운 파일의 포맷(주석, 들여쓰기 등)을 최대한 유지하면서 특정 필드만 교체합니다.
3. **디렉터리 동기화**: 스킬 디렉터리 내의 파일 추가, 삭제, 수정을 해시 기반으로 정합성을 맞춘 계획을 수립하고 실행합니다.

## 주요 구성 요소

### 1. SyncExecutor (`mod.rs`)
전체 동기화 프로세스를 제어하는 오케스트레이터입니다.
- `run()`: 설정 로드, 레지스트리 구축 후 `syncer::sync::Syncer`를 생성하여 모든 리소스에 대해 동기화를 수행합니다.

### 2. Syncer (`sync.rs`)
개별 리소스의 실제 동기화 로직을 수행하는 실행기입니다.
- `sync()`: 단일 리소스를 타겟에서 소스로 역변환(Detransform)을 수행하고 변경사항을 소스 파일에 반영합니다.

### 3. Patcher (`patcher.rs`)
마크다운 파일의 구조와 포맷을 최대한 보존하면서 특정 부분만 수정하는 역할을 수행합니다.
- **Surgical Update**: YAML Frontmatter의 특정 필드(`description` 등)나 본문(Body) 영역만 정규표현식을 사용하여 정교하게 교체합니다.
- **Format Preservation**: 전체 파일을 다시 쓰지 않고 변경이 필요한 라인만 수정함으로써 주석이나 들여쓰기 등의 스타일 유실을 방지합니다.

### 4. SyncPlanner (`planner.rs`)
두 디렉터리(Source와 Target)를 비교하여 수행해야 할 작업 목록(`SyncAction`)을 생성합니다.
- **Action Types**: `Add`, `Update`, `Delete`, `PatchMarkdown` (마크다운 특수 처리) 등의 액션을 도출합니다.
- **Change Detection**: 파일 존재 여부 및 SHA-256 해시 비교를 통해 변경 사항을 탐지합니다.

### 5. SkillSyncer (`skill.rs`)
`SyncPlanner`와 `Patcher`를 조율하여 실제 동기화 워크플로우를 실행하는 오케스트레이터입니다.
- **Workflow**: 계획 수립(`plan`) -> 액션 순회 -> 실제 파일 시스템 작업(복사, 삭제, 패치) 수행.

## 주요 데이터 구조

### `SyncAction`
디렉터리 비교 결과 도출된 개별 작업 단위입니다.
```rust
pub enum SyncAction {
    Add { relative_path: PathBuf, target_path: PathBuf },
    Update { relative_path: PathBuf, target_path: PathBuf },
    Delete { relative_path: PathBuf, source_path: PathBuf },
    PatchMarkdown { 
        relative_path: PathBuf, 
        source_path: PathBuf, 
        target_content: String 
    },
}
```

## 동기화 흐름

1. **컨텍스트 준비**: `SyncExecutor`가 `agb.yaml`을 로드하고 소스 레지스트리를 구축합니다.
2. **동기화기 생성**: 빌드 타겟에 맞는 `Transformer`와 실행 시점의 설정을 가진 `Syncer` 객체를 생성합니다.
3. **역변환 및 비교**: `Syncer::sync` 과정에서 `Transformer::detransform`을 통해 타겟 파일을 모델로 복원하고 소스와 비교합니다.
4. **계획 및 반영**: `patcher`, `planner`, `skill` 모듈을 통해 변경이 감지된 필드(Description, Content)를 업데이트하거나, 스킬 디렉터리의 전체 정합성을 맞춥니다.

## 설계 원칙
- **책임 분리**: "무엇을 할지 결정하는 로직(Planner)"과 "실제로 수행하는 로직(Syncer/SkillSyncer)", "텍스트를 조작하는 로직(Patcher)"을 분리하여 테스트 용이성을 높였습니다.
- **안전성**: 전체 파일을 무조건 덮어쓰는 대신, 필요한 경우에만 최소한의 변경을 가하여 데이터 손실 위험을 줄입니다.
