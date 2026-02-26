# Builder 모듈

## 개요
이 모듈은 `agb` 프로젝트의 핵심 빌드 오케스트레이션(Orchestration)을 담당합니다. `agb.yaml` 설정 파일을 기반으로 전체 빌드 파이프라인을 제어하며, 리소스 스캔부터 변환, 최종 파일 배포까지의 흐름을 관리합니다.

## 주요 구성 요소

### 1. Builder (`mod.rs`)
빌드 프로세스의 실제 실행 로직을 담고 있습니다. `Builder` 구조체는 다음과 같은 역할을 수행합니다:
- 빌드 환경(설정 파일 경로, 출력 디렉터리) 초기화
- 5단계 빌드 프로세스 실행 (`run()` 메서드)
- 각 단계별 상태 및 결과 로깅

### 2. Config (`config.rs`)
빌드 설정을 정의하고 관리합니다:
- `agb.yaml` 파일의 역직렬화(Deserialization) 및 구조화
- 빌드 대상 플랫폼(`core::BuildTarget`), 소스 경로, 제외 패턴, 대상 리소스 목록 정의
- `shellexpand`를 사용한 경로 내 물결표(`~`) 확장 지원

### 3. Emitter (`emitter.rs`)
변환된 최종 결과물을 물리적 파일로 출력합니다.
- **Clean**: 빌드 시작 전, 출력 디렉터리에서 이전 빌드의 잔재를 삭제하여 깨끗한 환경을 보장합니다.
- **Emit**: `core::TransformedFile` 목록을 바탕으로 파일을 기록합니다.

## 빌드 프로세스 (5단계)

1. **설정 로드 (Loading Config)**: `agb.yaml` 파일을 읽어 빌드 컨텍스트를 생성합니다.
2. **리소스 스캔 (Scanning Resources)**: 소스 디렉터리의 `plugins/` 구조를 분석하여 사용 가능한 모든 리소스를 수집합니다. (`loader` 모듈 사용)
3. **리소스 등록 및 필터링 (Registering)**: 수집된 리소스 중 설정 파일에 명시된 리소스만 선별하여 `core::Registry`에 등록합니다. 이 단계에서 타입과 이름을 조합하여 중복 여부를 검증합니다.
4. **변환 (Transforming)**: 
    - 선택된 타겟 플랫폼 규격에 맞게 리소스 내용을 변환합니다. (`transformer` 모듈 사용)
    - 소스 루트에 `AGENTS.md`가 존재할 경우, 타겟별 메인 지침 파일(예: `GEMINI.md`)로 자동 변환하여 포함합니다.
5. **배포 (Emitting)**: 변환된 결과물을 출력 디렉터리에 파일 형태로 저장합니다. (`Emitter` 사용)

## 사용 예시

```rust
use crate::builder::Builder;

fn main() -> anyhow::Result<()> {
    let builder = Builder::new("agb.yaml");
    builder.run()?;
    Ok(())
}
```
