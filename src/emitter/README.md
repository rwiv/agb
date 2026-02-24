# Emitter Module

`emitter` 모듈은 변환된 리소스 파일들을 파일 시스템에 실제로 기록하고, 빌드 전 대상 디렉터리를 정리하는 역할을 담당합니다.

## 핵심 역할

1. **빌드 디렉터리 정리(Clean)**: 새로운 빌드를 시작하기 전, 기존에 생성되었던 `commands/`, `agents/`, `skills/` 디렉터리와 메인 메모리 파일(`GEMINI.md`, `CLAUDE.md`, `OPENCODE.md` 등)을 삭제하여 깨끗한 상태를 만듭니다.
2. **파일 기록(Emit)**: `Transformer`에 의해 변환된 결과물(`TransformedFile`)들을 지정된 출력 경로에 물리적인 파일로 저장합니다.
3. **디렉터리 자동 생성**: 파일을 쓰기 전, 공통 유틸리티(`utils::fs`)를 사용하여 해당 파일이 위치할 디렉터리가 존재하지 않으면 자동으로 생성합니다.

## 모듈 구조

- `core.rs`: `Emitter` 구조체와 핵심 로직(`clean`, `emit`) 구현.
- `mod.rs`: 모듈 선언 및 `Emitter` 구조체 re-export.

## 주요 구성 요소

### 1. `Emitter` Struct (`core.rs`)
빌드 출력을 관리하는 메인 구조체입니다.

```rust
pub struct Emitter {
    output_path: PathBuf,
}

impl Emitter {
    /// 새로운 Emitter 인스턴스 생성
    pub fn new(output_path: impl Into<PathBuf>) -> Self;

    /// 기존 빌드 결과물 삭제
    pub fn clean(&self) -> Result<()>;

    /// 변환된 파일들을 파일 시스템에 기록
    pub fn emit(&self, files: &[TransformedFile]) -> Result<()>;
}
```

## 사용 예시

```rust
let emitter = Emitter::new("tests/fixtures");

// 1. 기존 파일 정리
emitter.clean()?;

// 2. 변환된 파일들 기록
let transformed_files = vec![/* ... */];
emitter.emit(&transformed_files)?;
```
