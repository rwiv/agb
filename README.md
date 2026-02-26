# agb (Agents Builder)

`agb`는 다양한 AI 코딩 에이전트를 위한 워크플로우 리소스(Commands, Agents, Skills)를 단일 소스에서 관리하고 빌드하는 CLI 도구입니다.

## 핵심 가치
- **중복 제로**: 단일 마크다운 소스로 여러 에이전트 지원.
- **명시적 관리**: `agb.yaml`을 통한 엄격한 리소스 제어.
- **이식성**: 플러그인 구조를 통한 쉬운 공유 및 재사용.

## 시작하기

### 설치 (Rust 환경 필요)
```bash
cargo install --path .
```

### 빌드 실행
`agb.yaml` 파일이 있는 디렉터리에서 실행합니다.
```bash
# 기본 설정(agb.yaml)으로 빌드
agb build

# 특정 설정 파일 지정
agb build --config custom-agb.yaml
```

## 상세 문서
상세한 내용은 `specs/` 디렉터리의 문서를 참조하십시오.

- [**PRD.md**](./specs/PRD.md): 프로젝트 목표 및 요구사항
- [**TECHSPEC.md**](./specs/TECHSPEC.md): 기술 규격, 설정 가이드 및 변환 규칙
- [**DESIGN.md**](./specs/DESIGN.md): 시스템 아키텍처 및 내부 설계

## 라이선스
MIT License
