# Syncer Diff Module

이 모듈은 타겟 에이전트 환경의 변경사항을 원본 소스로 동기화하기 위한 핵심 로직을 담당합니다. 파일 시스템 스캔, 변경 탐지, 그리고 마크다운의 수술적 업데이트(Surgical Update) 기능을 제공합니다.

## 핵심 컴포넌트

### 1. `MarkdownPatcher` (`markdown.rs`)
마크다운 파일의 구조와 포맷을 최대한 보존하면서 특정 부분만 수정하는 역할을 수행합니다.
- **Surgical Update**: YAML Frontmatter의 특정 필드(`description` 등)나 본문(Body) 영역만 정규표현식을 사용하여 정교하게 교체합니다.
- **Format Preservation**: 전체 파일을 다시 쓰지 않고 변경이 필요한 라인만 수정함으로써 주석이나 들여쓰기 등의 스타일 유실을 방지합니다.

### 2. `SyncPlanner` (`planner.rs`)
두 디렉터리(Source와 Target)를 비교하여 수행해야 할 작업 목록(`SyncAction`)을 생성합니다.
- **Action Types**: `Add`, `Update`, `Delete`, `PatchMarkdown` (마크다운 특수 처리) 등의 액션을 도출합니다.
- **Change Detection**: 파일 존재 여부 및 SHA-256 해시 비교를 통해 변경 사항을 탐지합니다.
- **Filtering**: `FileFilter`를 사용하여 `exclude` 패턴에 해당하는 파일을 제외합니다.

### 3. `SkillSyncer` (`mod.rs`)
`SyncPlanner`와 `MarkdownPatcher`를 조율하여 실제 동기화 워크플로우를 실행하는 오케스트레이터입니다.
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

## 사용 예시

```rust
use crate::syncer::diff::SkillSyncer;

// 스킬 디렉터리 동기화 실행
SkillSyncer::sync_skill_dir(source_path, target_path, &exclude_patterns)?;
```

## 설계 원칙
- **책임 분리**: "무엇을 할지 결정하는 로직(Planner)"과 "실제로 수행하는 로직(Syncer)", "텍스트를 조작하는 로직(Patcher)"을 분리하여 테스트 용이성을 높였습니다.
- **안전성**: 전체 파일을 무조건 덮어쓰는 대신, 필요한 경우에만 최소한의 변경을 가하여 데이터 손실 위험을 줄입니다.
