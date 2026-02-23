# Master Task List (TASKS.md): PomoTask

## Phase 1: 기반 설정 및 규약 정의 (Foundation & Specs)

이 단계는 서브에이전트들이 공통으로 사용할 환경과 뼈대를 구축합니다. (순차적 진행 필요)

- **Task 1.1: 프로젝트 초기화 및 라이브러리 세팅**
  - Next.js (App Router), Tailwind CSS, shadcn/ui 설치 및 기본 설정.
  - `src/app/layout.tsx`의 전역 레이아웃 구성.
- **Task 1.2: Supabase 연동 및 타입 정의**
  - `src/lib/supabase.ts` 클라이언트 셋업.
  - `TECH_SPEC.md`를 참조하여 `src/types/database.ts`에 DB 인터페이스(타입) 작성.
- **Task 1.3: 전역 상태 관리 및 Provider 셋업**
  - `src/store/useTimerStore.ts` (Zustand) 껍데기(초기 상태 및 액션 타입) 생성.
  - TanStack Query (React Query) Provider를 최상단 레이아웃에 적용.

## Phase 2: 독립적 UI 컴포넌트 개발 (UI & Mock)

데이터베이스 연결 없이 **Mock(가짜) 데이터**를 사용하여 화면을 먼저 그립니다. (서브에이전트 병렬 작업 가능)

- **Task 2.1: 인증 UI 구현** (`src/app/(auth)/login/page.tsx`)
  - 이메일/비밀번호 입력 폼 및 소셜 로그인 버튼 (UI만).
- **Task 2.2: 할 일(Task) 컴포넌트 구현** (`src/components/task/`)
  - 할 일 입력 폼 (`AddTaskForm`), 할 일 목록 (`TaskList`, `TaskItem`).
  - 더미 데이터를 사용하여 화면에 렌더링 확인.
- **Task 2.3: 포모도로 타이머 UI 구현** (`src/components/timer/`)
  - 남은 시간을 보여주는 타이머 디스플레이 컴포넌트.
  - 시작, 일시정지, 초기화 컨트롤 버튼.
- **Task 2.4: 대시보드 통계 UI 구현** (`src/app/dashboard/page.tsx`)
  - Recharts를 활용하여 최근 7일 집중 시간 바 차트(Bar Chart) 그리기 (Mock 데이터 사용).

## Phase 3: 비즈니스 로직 및 DB 연동 (Integration)

Phase 2에서 만든 UI에 실제 생명력(로직과 데이터)을 불어넣습니다.

- **Task 3.1: Supabase Auth 연동 적용**
  - Task 2.1의 로그인 폼에 실제 Supabase 로그인/회원가입 로직 연결.
  - 미인증 사용자 접근 제어 (Middleware 또는 컴포넌트 단 처리).
- **Task 3.2: 할 일(Task) CRUD 훅 작성 및 연결**
  - React Query를 사용하여 `useTasks`, `useCreateTask`, `useUpdateTask` 커스텀 훅 작성.
  - Task 2.2의 UI와 실제 훅 연결.
- **Task 3.3: 타이머 스토어 로직 완성 및 로그 기록 연동**
  - `useTimerStore.ts`의 실제 타이머 틱(Tick) 로직 구현 (`setInterval` 활용).
  - 타이머가 1사이클(25분) 완료될 때마다 `pomodoro_logs` 테이블에 기록(Insert)하는 로직 연결.

## Phase 4: 마감 및 고도화 (Refinement)

- **Task 4.1: 에러 핸들링 및 알림 추가**
  - 모든 API 요청 실패 시 토스트(Toast) 알림 UI 띄우기.
- **Task 4.2: 타이머 종료 시각/청각 피드백 추가**
  - 타이머 완료 시 브라우저 알림(Notification API) 또는 알림음 재생 추가.
