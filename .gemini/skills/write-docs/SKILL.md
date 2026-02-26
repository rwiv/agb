---
name: write-docs
description: Use this skill when creating, updating, or maintaining project documentation (README, PRD, TECHSPEC, DESIGN) or when writing technical designs, implementation plans, and tasks in the 'plans/' directory to ensure architectural consistency and adherence to established documentation standards.
---

# Skill: write-docs

이 스킬은 `agb` 프로젝트의 문서 체계(PRD, TECHSPEC, DESIGN) 및 구현 계획 문서(DESIGN, PLAN, TASK)를 작성하고 유지보수할 때 사용합니다. 일관성 있고 고품질의 문서를 작성하기 위한 표준 가이드를 제공합니다.

## 1. 사용 시점 (When to Use)

- 프로젝트의 새로운 요구사항이나 비즈니스 가치를 정의할 때 (`PRD.md`)
- 외부 인터페이스, 설정 규격, 변환 규칙을 명시할 때 (`TECHSPEC.md`)
- 시스템 아키텍처나 내부 모듈 설계를 정의할 때 (`DESIGN.md`)
- 특정 기능을 구현하거나 리팩터링하기 위한 기술 설계 및 계획을 세울 때 (`plans/`)
- 모듈별 역할과 책임을 문서화할 때 (`src/*/README.md`)

## 2. 핵심 원칙 (Core Mandates)

1.  **단일 진실 공급원 (SSOT)**: 동일한 정보가 여러 문서에 중복되지 않게 합니다. 규격은 `TECHSPEC`에, 구현 방식은 `DESIGN`에 작성하고 상호 링크를 활용합니다.
2.  **계층적 구조**: 요구사항(`PRD`) -> 기술 규격(`TECHSPEC`) -> 내부 설계(`DESIGN`) 순으로 구체화합니다.
3.  **시각화 우선**: 복잡한 의존성, 흐름, 아키텍처는 반드시 `Mermaid` 다이어그램을 활용하여 설명합니다.
4.  **최신화 유지**: 기능 변경 시 코드와 함께 문서를 즉시 업데이트합니다. 모든 PR은 관련 문서의 업데이트를 포함해야 합니다.

## 3. 문서별 작성 가이드

### 3.1. 전역 및 명세 문서 (Global & Spec Documents)

- **`README.md` (Gateway)**: 프로젝트 요약, Quick Start(설치/실행), 문서 인덱스 제공.
- **`specs/PRD.md` (Requirements)**: "What & Why". 비즈니스 배경, 주요 요구사항, 성공 지표 정의. 기술적 예시 배제.
- **`specs/TECHSPEC.md` (Technical Spec)**: "How it behaves". 외부 인터페이스(YAML Schema), 리소스 규칙, 변환/병합 알고리즘 규격 정의.
- **`specs/DESIGN.md` (Internal Design)**: "How it works". 시스템 아키텍처, 모듈 구조(Mermaid), 핵심 데이터 모델 설계.

### 3.2. 구현 계획 문서 (`plans/` Documents)

- **`DESIGN.md` (Tech Design)**: 개별 기능/리팩터링의 구체적인 기술적 접근 방식, 아키텍처 변경, 데이터 흐름 정의.
- **`PLAN.md` (Roadmap)**: 단계별(Phases) 목표 및 작업(Tasks) 목록, 의존성 명시.
- **`TASK-X-Y.md` (Actionable Task)**: 원자적 작업 지침. 수정 파일, 구체적인 구현 로직(코드 블록 활용), 검증 방법(테스트) 포함.

## 4. 문서 작성 워크플로우

1.  **컨텍스트 파악**: 수정하려는 내용이 요구사항인지, 규격인지, 혹은 내부 구현 계획인지 확인합니다.
2.  **문서 선택**: 위 분류에 따라 수정하거나 생성할 문서를 결정합니다.
3.  **작성 표준 적용**: `write-docs`의 표준 형식을 준수하여 내용을 작성합니다. 특히 `TASK` 문서는 추가 질문 없이 구현이 가능할 정도로 구체적이어야 합니다.
4.  **상호 참조 검증**: 관련 문서 간의 링크가 올바른지, 정보의 중복이나 충돌은 없는지 확인합니다.
