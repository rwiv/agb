# Technical Specification: agb (Agents Builder)

## 1. 기술 스택 (Technical Stack)

- **언어:** Rust (Edition 2021)
- **CLI 프레임워크:** `clap` (v4, derive 기능 사용)
- **직렬화/역직렬화:**
  - `serde`: 데이터 모델링 공통
  - `serde_yaml`: `agb.yaml` 파싱
  - `serde_json`: 리소스 메타데이터(`*.json`) 파싱
  - `toml`: Gemini-cli용 결과물 생성
- **파일 시스템 및 유틸리티:**
  - `walkdir`: 플러그인 디렉터리 재귀 탐색
  - `glob`: `agb.yaml`의 exclude 패턴 매칭
  - `anyhow`: 애플리케이션 레벨 에러 처리
  - `thiserror`: 라이브러리/코어 레벨 커스텀 에러 정의

## 2. 시스템 아키텍처 (Architecture)

`agb`는 **파이프라인 아키텍처**를 따르며, 각 단계는 독립적인 모듈로 분리됩니다. 특히 에이전트별 변환 로직은 `Transformer` 트레이트를 통해 확장 가능하도록 설계합니다.

### 2.1 데이터 흐름 (Data Flow)
1. **Load**: `agb.yaml`을 읽고, 명시된 `source` 경로 하위의 `plugins/` 디렉터리에서 대상 리소스를 스캔하여 메모리에 로드합니다.
2. **Validate**: 리소스 이름 충돌 여부 및 필수 메타데이터 존재 여부를 검증합니다.
3. **Transform**: 선택된 `target`에 해당하는 `Transformer` 구현체가 리소스를 대상 규격에 맞게 변환합니다.
4. **Emit**: `agb.yaml`이 위치한 출력 디렉터리를 정리(Clean)하고 변환된 파일들을 물리적 경로에 작성합니다.

### 2.2 핵심 트레이트: `Transformer`

새로운 에이전트 지원을 위해 아래와 같은 인터페이스를 제공합니다.

```rust
pub trait Transformer {
    /// 리소스를 타겟 포맷으로 변환
    fn transform(&self, resource: &Resource) -> Result<TransformedFile>;
    
    /// 전역 지침(AGENTS.md)을 타겟 규격으로 변환
    fn transform_root_prompt(&self, content: &str) -> Result<TransformedFile>;
}
```

## 3. 프로젝트 구조 (Module Structure)

```text
src/
├── main.rs          # CLI 엔트리포인트 (명령어 파싱 및 오케스트레이션)
├── config.rs        # agb.yaml 모델 및 로드 로직
├── core/
│   ├── mod.rs
│   ├── loader.rs    # 파일 시스템 스캔 및 Resource 객체 생성
│   ├── registry.rs  # 리소스 저장소 및 충돌 검사 로직
│   └── resource.rs  # Resource, Command, Agent, Skill 데이터 모델
├── transformers/
│   ├── mod.rs       # Transformer 트레이트 정의 및 Factory
│   ├── gemini.rs    # Gemini-cli (TOML 기반) 구현
│   ├── claude.rs    # Claude-code (Markdown 기반) 구현
│   └── opencode.rs  # OpenCode 구현
└── emitter/
    ├── mod.rs       # 파일 쓰기 및 디렉터리 관리 (Clean & Write)
    └── fs_utils.rs  # 파일 시스템 유틸리티
```

## 4. 상세 설계 고려 사항

### 4.1 리소스 식별 및 충돌 해결
- 리소스는 내부적으로 `plugin_name:resource_name`으로 식별됩니다.
- 빌드 결과물은 플랫(Flat)하게 생성되므로, `resource_name`이 동일한 리소스가 서로 다른 플러그인에서 선택될 경우 `Registry` 단계에서 에러를 발생시킵니다.

### 4.2 Gemini-cli 변환 특이사항
- `commands/[name].md`의 마크다운 전문은 TOML의 `prompt` 필드로 들어갑니다.
- `commands/[name].json`의 메타데이터 필드들은 TOML의 최상위 키로 매핑됩니다.

### 4.3 에러 처리 전략
- `agb.yaml`이 없는 경우: "Config file not found" 에러와 함께 종료.
- 변환 실패 시: 해당 리소스 이름과 실패 원인을 명시하여 에러 출력.
- 빌드 디렉터리 삭제 실패 시: 권한 문제 등을 사용자에게 알림.

## 5. 단계별 구현 계획

1. **Phase 1: CLI & Config**: `clap` 기반 인터페이스 및 `agb.yaml` 파싱 구현.
2. **Phase 2: Core Loader**: 플러그인 디렉터리 스캔 및 리소스 로딩 로직 완성.
3. **Phase 3: Transformer Interface**: 트레이트 정의 및 `Gemini` 변환기 우선 구현.
4. **Phase 4: Emitter & Integration**: 파일 쓰기 로직 구현 및 전체 파이프라인 통합.
5. **Phase 5: Multi-target Support**: `Claude`, `OpenCode` 변환기 추가 및 테스트.
