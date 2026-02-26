# DESIGN: 멀티라인 Description 동기화 방지 및 에러 처리

## 1. 배경 및 목적
현재 `MdPatcher`는 `description:` 필드를 정규표현식 기반의 라인 단위 교체 방식으로 처리합니다. 이 방식은 YAML의 멀티라인 마커(`|`, `>`)나 들여쓰기를 통한 멀티라인 데이터를 인식하지 못하며, 이 경우 `sync` 수행 시 원본 YAML 구조가 파괴되는 버그가 발생합니다.

복잡한 YAML 파싱 라이브러리를 도입하여 스타일을 보존하면서 멀티라인을 처리하는 대신, **멀티라인이 감지되면 명시적으로 에러를 발생시켜 원본 데이터의 파손을 방지**하는 것을 목적으로 합니다.

## 2. 핵심 변경 사항

### 2.1 MdPatcher::update_description 인터페이스 변경
- **반환 타입**: `()` -> `anyhow::Result<()>`
- **에러 조건**:
    - **입력 데이터**: 새로 동기화하려는 설명(`new_desc`)에 줄바꿈(`
`)이 포함된 경우.
    - **원본 데이터**: 원본의 `description:` 키가 있는 라인이 `|` 또는 `>`로 끝나는 경우.
    - **구조적 감지**: `description:` 키가 있는 라인의 다음 줄이 공백(Space)으로 시작하는 경우 (들여쓰기된 멀티라인 데이터).

### 2.2 Syncer 및 SkillSyncer 호출부 대응
- `update_description`이 `Result`를 반환하므로, 상위 호출부에서 이를 `?` 연산자로 처리하여 동기화 작업을 중단하고 사용자에게 에러 메시지를 전달합니다.

## 3. 상세 알고리즘 (Pseudo-code)

```rust
pub fn update_description(&mut self, new_desc: &str) -> Result<()> {
    // 1. 입력 값 검증
    if new_desc.contains('
') {
        bail!("Multi-line description is not supported for synchronization.");
    }

    // ... (YAML 영역 파싱) ...

    for (i, line) in lines.iter().enumerate() {
        if re.is_match(line) {
            // 2. 원본 마커 감지 (| 또는 >)
            let trimmed = line.trim();
            if trimmed.ends_with('|') || trimmed.ends_with('>') {
                bail!("Multi-line description marker (| or >) detected in source. Sync aborted to prevent corruption.");
            }

            // 3. 다음 줄 들여쓰기 감지
            if i + 1 < lines.len() && lines[i+1].starts_with(' ') {
                bail!("Indented multi-line description detected in source. Sync aborted to prevent corruption.");
            }

            // ... (정상적인 단일 라인 교체) ...
        }
    }
}
```

## 4. 기대 효과
- 잘못된 YAML 생성으로 인한 데이터 오염 방지.
- 사용자에게 명확한 에러 메시지 제공을 통해 수동 수정 유도.
- 시스템 복잡도 증가 없이 안정성 확보.
