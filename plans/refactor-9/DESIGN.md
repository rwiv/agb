# Design: Syncer Refactoring (Patcher, Planner, Syncer)

## 1. 개요 (Overview)
현재 `src/syncer/diff.rs`는 파일 시스템 스캔, 변경 사항 탐지, 마크다운 텍스트 조작 로직이 하나의 파일에 혼재되어 있습니다. 이는 코드의 가독성을 떨어뜨리고 단위 테스트를 어렵게 만듭니다.

본 설계는 책임을 명확히 분리하여 유지보수성과 테스트 용이성을 높이는 것을 목표로 합니다.

## 2. 핵심 컴포넌트 (Core Components)

### 2.1. `MarkdownPatcher` (Text Manipulation)
마크다운 원본의 형식을 최대한 보존하면서 특정 필드(예: `description`)나 본문(Body)만 "패치"하는 역할을 담당합니다. 정규표현식과 라인 단위 처리를 사용하여 스타일 파괴를 최소화합니다.

- **역할**: 마크다운 Frontmatter 및 Body 수술적 수정.
- **주요 기능**:
  - `update_field(key, value)`: 특정 키의 값을 교체.
  - `replace_body(new_content)`: Frontmatter를 유지한 채 본문만 교체.

### 2.2. `SyncPlanner` (Change Detection)
두 디렉터리(Source, Target)를 비교하여 수행해야 할 작업 목록(`DiffAction`)을 생성합니다. 실제 파일 시스템을 수정하지 않고 계획만 세우므로 테스트가 용이합니다.

- **역할**: 디렉터리 비교 및 액션 계획 수립.
- **주요 데이터 구조**:
  ```rust
  pub enum SyncAction {
      Add { relative_path: PathBuf, source: PathBuf },
      Update { relative_path: PathBuf, source: PathBuf },
      Delete { relative_path: PathBuf },
      UpdateMarkdown { relative_path: PathBuf, source: PathBuf, target_content: String },
  }
  ```

### 2.3. `SkillSyncer` (Orchestrator)
`SyncPlanner`로부터 계획을 받아 실제 파일 시스템 작업을 수행하고, 필요시 `MarkdownPatcher`를 호출하여 정교한 업데이트를 수행합니다.

- **역할**: 동기화 워크플로우 실행 및 조율.

## 3. 새로운 모듈 구조 (New Module Structure)

```text
src/syncer/
├── mod.rs
├── sync.rs
└── diff/
    ├── mod.rs          # SkillSyncer (Orchestrator)
    ├── markdown.rs     # MarkdownPatcher (Text Patcher)
    └── planner.rs      # SyncPlanner (Action Planner)
```

## 4. 데이터 흐름 (Data Flow)

1. **Plan Phase**: `SkillSyncer`가 `SyncPlanner`를 호출하여 `Source`와 `Target` 디렉터리 간의 `Vec<SyncAction>`을 생성합니다.
2. **Execute Phase**: `SkillSyncer`가 `SyncAction` 목록을 순회하며 실행합니다.
   - 일반 파일: `fs::copy` 또는 `fs::remove_file` 수행.
   - 마크다운 파일 (`SKILL.md` 등): `MarkdownPatcher`를 로드하여 필요한 필드/본문만 패치 후 저장.

## 5. 기대 효과 (Benefits)
- **테스트 용이성**: `SyncPlanner`와 `MarkdownPatcher`를 파일 시스템 의존성 없이 독립적으로 테스트할 수 있습니다.
- **가독성**: "무엇을 할지 결정하는 로직"과 "실제로 수행하는 로직"이 분리되어 흐름 파악이 쉬워집니다.
- **안전성**: 전체 파일을 덮어쓰는 대신 필요한 부분만 수정함으로써 예기치 않은 데이터 손실이나 포맷 훼손을 방지합니다.
