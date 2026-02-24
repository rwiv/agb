# Technical Specification: agb (Agents Builder)

## 1. 기술 스택 (Technical Stack)

- **언어:** Rust (Edition 2024)
- **CLI 프레임워크:** `clap` (v4.5, derive 기능 사용)
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

`agb`는 **파이프라인 아키텍처**를 따르며, `builder` 모듈이 전체 공정을 오케스트레이션합니다. 각 단계는 독립적인 모듈로 분리되어 있으며, 에이전트별 변환 로직은 `Transformer` 트레이트를 통해 확장 가능하도록 설계되었습니다.

### 2.1 데이터 흐름 (Data Flow)
1. **Load Config**: `agb.yaml`을 읽어 빌드 컨텍스트를 생성합니다. (`builder/config.rs`)
2. **Scan & Load**: 소스 경로의 플러그인을 스캔하고 `Resource` 객체로 로드합니다. (`resource/loader.rs`)
3. **Validate & Register**: 리소스 이름 충돌을 검증하고 레지스트리에 등록합니다. (`resource/registry.rs`)
4. **Transform**: 선택된 타겟에 맞는 `Transformer`가 리소스를 변환합니다. (`transformers/`)
5. **Emit**: 기존 결과물을 정리하고 변환된 파일을 물리적 경로에 작성합니다. (`resource/emitter.rs`)

### 2.2 핵심 트레이트: `Transformer` (`src/transformers/base.rs`)

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

| 모듈 경로 | 설명 | 상세 문서 |
| :--- | :--- | :--- |
| `src/main.rs` | CLI 엔트리포인트 및 실행 제어 | - |
| `src/builder/` | 빌드 파이프라인 오케스트레이션 및 설정 관리 | [README.md](../../src/builder/README.md) |
| `src/resource/` | 리소스 데이터 모델, 스캔, 레지스트리 및 배포(Emitter) | [README.md](../../src/resource/README.md) |
| `src/transformers/` | 에이전트별 포맷 변환 로직 (Gemini, Claude 등) | [README.md](../../src/transformers/README.md) |
| `src/utils/` | 공통 유틸리티 (FS 조작 등) | - |

### 3.1 세부 파일 구성
- **builder**: `core.rs` (프로세스 제어), `config.rs` (agb.yaml 파싱)
- **resource**: `types.rs` (모델, `TransformedFile` 포함), `loader.rs` (스캔), `registry.rs` (중복 검증), `emitter.rs` (출력 로직)
- **transformers**: 
  - `base.rs` (트레이트), `factory.rs` (생성기)
  - `providers/`: `gemini.rs`, `claude.rs`, `opencode.rs` (구현체)
- **utils**: `fs.rs` (파일 시스템 유틸리티)

## 4. 상세 설계 고려 사항

### 4.1 리소스 식별 및 충돌 해결
- 리소스는 내부적으로 `plugin_name:resource_name`으로 식별됩니다.
- 빌드 결과물은 플랫(Flat)하게 생성되므로, 동일한 `resource_name`을 가진 리소스가 서로 다른 플러그인에서 선택될 경우 `Registry` 단계에서 에러를 발생시켜 안전성을 보장합니다.

### 4.2 Gemini-cli 변환 특이사항
- `commands/[name].md`의 마크다운 전문은 TOML의 `prompt` 필드로 삽입됩니다.
- `commands/[name].json`의 메타데이터 필드들은 TOML의 최상위 키로 매핑됩니다.

### 4.3 에러 처리 전략
- `agb.yaml` 미존재 시: 사용자 친화적인 에러 메시지와 함께 종료.
- 리소스 충돌 시: 충돌이 발생한 플러그인과 리소스 이름을 명시.
- 빌드 전 Clean 실패 시: 권한 문제 등을 상세히 보고.
