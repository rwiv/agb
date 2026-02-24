# TASK-1-3: 메인 함수 재구성 및 검증

## 1. 개요
추출된 함수들을 조합하여 `load_resources` 함수를 깔끔한 파이프라인 구조로 재구성합니다.

## 2. 작업 내용
- `load_resources` 함수의 본문을 새로 만든 함수 호출로 대체
- 기존의 장황한 루프를 `map`과 `collect`를 사용하는 선언적 스타일로 변경 시도

```rust
pub fn load_resources<P: AsRef<Path>>(root: P, files: Vec<PathBuf>) -> Result<Vec<Resource>> {
    let root = root.as_ref();
    let groups = group_files_by_resource(root, files)?;
    
    groups.into_iter()
        .map(|(key, paths)| parse_resource(key, paths))
        .collect()
}
```

## 3. 검증 계획
- `cargo test`를 실행하여 기존 e2e 테스트 및 유닛 테스트 통과 확인
- 빌드 결과물이 리팩토링 이전과 동일한지 확인
