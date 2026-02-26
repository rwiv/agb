# Builder 모듈

## 개요
`builder` 모듈은 `agb` 프로젝트의 리소스 빌드 유틸리티를 제공합니다. `app` 모듈에 의해 호출되어 변환된 리소스를 실제 파일로 배포(Emission)하는 역할을 담당합니다.

## 주요 구성 요소

### 1. Emitter (`emitter.rs`)
변환된 최종 결과물을 물리적 파일로 출력합니다.
- **Clean**: 빌드 시작 전, 출력 디렉터리에서 이전 빌드의 잔재를 삭제하여 깨끗한 환경을 보장합니다.
- **Emit**: `core::TransformedResource` 목록을 바탕으로, 텍스트 변환된 파일(`files`)들은 디스크에 기록하고 단순 포함 파일(`extras`)들은 물리적으로 대상 디렉터리에 복사합니다.

### 2. Builder (`mod.rs`)
빌드 인스턴스를 관리하며, `app::App`에서 빌드 로직을 수행할 때 보조적인 역할을 수행합니다.

## 빌드 프로세스에서의 역할
`app` 모듈의 `App::build` 메서드가 `loader`와 `transformer`를 이용해 변환된 리소스 목록을 생성하면, `builder::Emitter`가 이를 최종적으로 파일 시스템에 기록합니다.

## 사용 예시

```rust
use crate::builder::emitter::Emitter;

let emitter = Emitter::new(output_dir);
emitter.clean()?;
emitter.emit(&transformed_resources)?;
```
