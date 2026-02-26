# Task 2-1: `SyncPlanner` 구현

## 1. 개요
두 디렉터리를 비교하여 추가, 수정, 삭제가 필요한 파일 목록을 도출하는 `SyncPlanner`를 구현합니다.

## 2. 작업 상세

### 2.1. 파일 생성
- `src/syncer/diff/planner.rs` 파일을 생성합니다.

### 2.2. `SyncAction` 열거형 및 `SyncPlanner` 정의
```rust
pub enum SyncAction {
    Add { relative_path: PathBuf, target_path: PathBuf },
    Update { relative_path: PathBuf, target_path: PathBuf },
    Delete { relative_path: PathBuf, source_path: PathBuf },
    // SKILL.md 등 특수 처리가 필요한 경우
    PatchMarkdown { 
        relative_path: PathBuf, 
        source_path: PathBuf, 
        target_content: String 
    },
}

pub struct SyncPlanner {
    filter: FileFilter,
}

impl SyncPlanner {
    pub fn new(exclude_patterns: &[String]) -> Result<Self> { ... }
    
    /// source와 target 디렉터리를 비교하여 액션 목록 생성
    pub fn plan(&self, source_dir: &Path, target_dir: &Path) -> Result<Vec<SyncAction>> { ... }
}
```

### 2.3. 비교 로직 이주
- `src/syncer/diff.rs`의 `sync_skill_dir` 함수 내부에 있던 루프와 조건문(해시 비교, exclude 체크 등)을 `plan` 메서드로 옮깁니다.

## 3. 검증 방법
- `cargo test syncer::diff::planner`를 수행하여 다양한 시나리오(신규 파일, 수정된 파일, 삭제된 파일)에서 올바른 `SyncAction`이 생성되는지 확인합니다.
