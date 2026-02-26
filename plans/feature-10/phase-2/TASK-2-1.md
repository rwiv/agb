# TASK 2-1: Extend `Transformer` Trait with `detransform`

## 개요 (Description)
`transformer::Transformer` 트레이트에 타겟 빌드 결과물을 공용 `ResourceData` 모델로 다시 복원하는 `detransform` 메서드를 추가합니다.

## 수정 파일 (Files to Modify)
- `src/transformer/mod.rs`

## 상세 지침 (Actionable Instructions)
1. `Transformer` 트레이트에 `detransform(&self, r_type: ResourceType, file_content: &str) -> Result<ResourceData>` 메서드를 추가합니다.
2. `detransform` 시에는 `name`과 `plugin` 정보가 없으므로 빈 문자열로 초기화하고, `content`와 `metadata`는 타겟 파일로부터 파싱합니다.
3. `source_path`는 `detransform` 시점에는 알 수 없으므로 기본값(`PathBuf::new()`)을 저장합니다. (나중에 `Syncer`가 Registry의 정보를 덮어쓰거나 수정할 때 활용)
4. 모든 구현체(`GeminiTransformer`, `DefaultTransformer`)가 이 메서드를 구현하도록 서명을 업데이트합니다.

## 검증 방법 (Verification)
- `cargo check`를 실행하여 트레이트 규격 변경으로 인한 컴파일 에러를 확인합니다.
- 메서드 추가에 따른 컴파일 성공 여부만 먼저 확인합니다.
