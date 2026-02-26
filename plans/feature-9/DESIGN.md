# DESIGN: Skill Extra Files Inclusion & Core Model Refactoring

## 1. 개요
Skill 디렉터리 내 추가 파일 포함 기능을 구현함과 동시에, `core` 모듈에 부적절하게 위치했던 로더 전용 모델들을 정리합니다. 시스템의 책임을 명확히 하고 도메인 모델의 순수성을 확보합니다.

## 2. 변경 사항

### 2.1 Core 데이터 모델 정리 (`src/core/model.rs`)
- **도메인 모델 유지**: `Resource`, `ResourceData`, `TransformedFile` 등 최종 결과물 모델만 유지.
- **삭제/이동**: `ResourceKey`, `ResourcePaths`를 삭제 (로더 내부로 이동).
- **새로운 모델**:
    - **`ExtraFile`**: 복사 대상 정보 (`source`, `target`).
    - **`TransformedResource`**: 빌드된 리소스 단위 (`files: Vec<TransformedFile>`, `extras: Vec<ExtraFile>`).
    - **`SkillData`**: `base: ResourceData`, `extras: Vec<ExtraFile>` 포함.

### 2.2 Loader 내부 모델 구축 (`src/loader/`)
- **`ScannedResource` (신규)**: `ResourceKey`와 `ResourcePaths`를 하나로 통합한 로더 전용 구조체.
  ```rust
  pub struct ScannedResource {
      pub plugin: String,
      pub name: String,
      pub paths: ScannedPaths,
  }

  pub enum ScannedPaths {
      Command { md: Option<PathBuf>, metadata: Option<PathBuf> },
      Agent { md: Option<PathBuf>, metadata: Option<PathBuf> },
      Skill { md: Option<PathBuf>, metadata: Option<PathBuf>, extras: Vec<PathBuf> },
  }
  ```
- **캡슐화**: 파일 스캔 및 그룹화 단계의 모든 중간 데이터는 `loader` 외부로 노출되지 않음.

### 2.3 Builder & Emitter
- **Builder**: `Transformer`의 결과와 `Resource` 내의 `extras`를 결합하여 `TransformedResource` 생성.
- **Emitter**: `TransformedResource` 목록을 받아 물리적 작업(Write/Copy) 수행.

## 3. 데이터 흐름
1. **Scan (Loader)**: 파일을 탐색하여 `ScannedResource` 목록 생성.
2. **Parse (Loader)**: `ScannedResource`를 읽어 `Resource` 객체로 변환 (Skill은 `ExtraFile` 목록 포함).
3. **Build (Builder)**: `Resource`를 변환하고 `TransformedResource`로 캡슐화.
4. **Emit (Emitter)**: 구조화된 데이터를 기반으로 파일 시스템 작업 수행.
