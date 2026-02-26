# DESIGN: Syncer 모듈 구조 고도화 (Refactor-12)

## 1. 개요
현재 `syncer` 모듈의 복잡도를 낮추고, 확장성을 높이며, 운영 편의성을 위해 로깅 방식을 개선합니다.

## 2. 주요 개선 사항

### 2.1 DirectorySyncer 도입 (구 SkillSyncer 일반화)
- **목적**: 스킬뿐만 아니라 향후 추가될 수 있는 디렉토리 기반 리소스의 동기화를 범용적으로 처리합니다.
- **설계**: `src/syncer/skill.rs`를 `src/syncer/directory.rs`로 이동 및 이름을 `DirectorySyncer`로 변경합니다.
- **기능**: `SyncPlanner`를 사용하여 두 디렉토리 간의 차이점을 분석하고 파일을 동기화하는 역할에 집중합니다.

### 2.2 Resource 모델 다형성 강화 (Helper Methods 추가)
- **목적**: `Syncer::sync_resource` 내의 타입별 분기 로직(`match resource`)을 줄여 코드 가독성을 높입니다.
- **설계**: `Resource` Enum에 다음과 같은 헬퍼 메서드를 추가합니다.
    - `main_source_path()`: 동기화 대상 마크다운 파일의 전체 경로 반환.
    - `metadata()`: 리소스의 메타데이터 반환.
    - `content()`: 리소스의 현재 본문 반환.
- **효과**: `Syncer`는 리소스의 구체적인 구조를 몰라도 동일한 인터페이스로 동기화를 수행할 수 있습니다.

### 2.3 표준 로깅 도입
- **목적**: `println!` 기반의 출력을 지양하고, 표준 `log` 크레이트를 사용하여 로그 레벨에 따른 제어가 가능하도록 합니다.
- **설계**:
    - `Cargo.toml`에 `log` 및 `env_logger` (또는 유사체) 추가.
    - `syncer` 모듈 내의 모든 `println!`을 `info!`, `debug!`, `warn!` 등으로 교체.
- **효과**: CLI 사용자에게는 필요한 정보만 노출하고, 개발/디버깅 시에는 상세 로그를 확인할 수 있습니다.

## 3. 구조적 변화 예시

### AS-IS (Syncer 로직)
```rust
let (source_path, current_metadata) = match resource {
    Resource::Command(d) | Resource::Agent(d) => (&d.source_path, &d.metadata),
    Resource::Skill(s) => (&s.base.source_path, &s.base.metadata),
};

let source_file_content = match resource {
    Resource::Command(_) | Resource::Agent(_) => fs::read_to_string(source_path)?,
    Resource::Skill(_) => fs::read_to_string(source_path.join(SKILL_MD))?,
};
```

### TO-BE (Syncer 로직)
```rust
let source_path = resource.main_source_path();
let source_file_content = fs::read_to_string(&source_path)?;
let current_metadata = resource.metadata();
```
