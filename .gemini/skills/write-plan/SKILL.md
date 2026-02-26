---
name: write-plan
description: Use this skill when initiating a new feature, bug fix, or refactoring task to establish a clear implementation roadmap. This skill mandates a 'plan-before-code' workflow, guiding the development of technical designs (DESIGN.md), strategic project plans (PLAN.md), and actionable task-level documentation to ensure thorough verification and alignment before execution.
---

코드를 작성하기 이전에 계획 문서를 작성하시오.

- 충분한 코드베이스 리서치 작업을 수행한 뒤 문서를 작성하시오.
- 복잡한 내용을 다루는 계획이라면 `PLAN.md` 문서 작성 이전에 `DESIGN.md` 문서를 작성하시오.
- 개별 task를 작성하기 이전에 `PLAN.md` 문서가 존재하는지 체크하고, 만약 없다면 `PLAN.md` 문서를 작성하시오.
- `PLAN.md`를 기반으로 하여 구체적인 tasks 문서들을 작성하시오.

## 작성 원칙

- 말로 설명하기 복잡한 내용은 코드 블럭으로 표현하는 것이 가독성이 높아진다.
- 문서 작성만 수행하시오. 구현은 사용자의 허가가 떨어진 이후에 수행해야한다.

## 참고 자료
- `DESIGN.md`
    - @plans/refactor-6/DESING.md
- `PLAN.md`
    - @plans/feature-1/PLAN.md
    - @plans/refactor-6/PLAN.md
- `<task-name>.md`
    - @plans/feature-1/phase-1/TASK-1-2.md
    - @plans/refactor-6/phase-1/TASK-1-2.md
