# Builder 모듈

## 개요
이 모듈은 `agb` 프로젝트의 핵심 빌드 오케스트레이션(Orchestration)을 담당합니다. `agb.yaml` 설정 파일을 기반으로 전체 빌드 파이프라인을 제어하며, 리소스 스캔부터 변환, 최종 파일 배포까지의 흐름을 관리합니다.

## 주요 구성 요소

### 1. Module Entry (`mod.rs`)
모듈의 진입점으로, 서브모듈(`builder`, `config`)을 선언하고 외부에서 사용하기 편리하도록 `Builder` 구조체를 리익스포트(Re-export)합니다.

### 2. Builder (`builder.rs`)
빌드 프로세스의 실제 실행 로직을 담고 있습니다. `Builder` 구조체는 다음과 같은 역할을 수행합니다:
- 빌드 환경(설정 파일 경로, 출력 디렉토리) 초기화
- 5단계 빌드 프로세스 실행 (`run()` 메서드)
- 각 단계별 상태 및 결과 로깅

### 3. Config (`config.rs`)
빌드 설정을 정의하고 관리합니다:
- `agb.yaml` 파일의 역직렬화(Deserialization) 및 구조화
- 빌드 대상 플랫폼(`BuildTarget`), 소스 경로, 제외 패턴, 대상 리소스 목록 정의
- 설정 파일 로드 및 파싱 로직 제공

## 빌드 프로세스 (5단계)

1. **설정 로드 (Loading Config)**: `agb.yaml` 파일을 읽어 빌드 컨텍스트를 생성합니다.
2. **리소스 스캔 (Scanning Resources)**: 소스 디렉토리의 플러그인 구조를 분석하여 사용 가능한 모든 리소스를 수집합니다.
3. **리소스 등록 및 필터링 (Registering)**: 수집된 리소스 중 설정 파일에 명시된 리소스만 선별하여 `Registry`에 등록합니다.
4. **변환 (Transforming)**: 선택된 타겟 플랫폼(Gemini, Claude, OpenCode 등)의 규격에 맞게 리소스 내용을 변환합니다.
5. **배포 (Emitting)**: 변환된 결과물을 출력 디렉토리에 파일 형태로 저장하고 이전 결과물을 정리합니다.

## 사용 예시

```rust
use crate::builder::Builder;

fn main() -> anyhow::Result<()> {
    // Builder 인스턴스 생성 (설정 파일 경로 전달)
    let builder = Builder::new("agb.yaml");
    
    // 빌드 프로세스 실행
    builder.run()?;
    
    Ok(())
}
```
