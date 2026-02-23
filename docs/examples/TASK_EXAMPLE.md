# Task 2.3: 포모도로 타이머 UI 컴포넌트 구현

## 1. Objective (목표)

- 사용자가 남은 시간을 확인하고 타이머를 제어(시작/일시정지/초기화)할 수 있는 직관적인 타이머 UI 컴포넌트들을 개발합니다.
- **주의:** 이 단계에서는 실제 `setInterval` 타이머 틱(Tick) 로직이나 Zustand 전역 상태 연동은 수행하지 않습니다. **오직 화면(UI) 렌더링, 상태 인터페이스 정의, 컴포넌트 분리**에만 집중하세요.

## 2. Context & Files (작업 범위)

- **읽기 전용 (참고용):**
  - `docs/TECH_SPEC.md` (디자인 시스템 및 색상 규칙 확인)
  - `src/components/ui/button.tsx` (프로젝트에 세팅된 shadcn/ui 공통 버튼 컴포넌트)
- **생성 및 수정할 파일:**
  - `src/components/timer/TimerDisplay.tsx`
  - `src/components/timer/TimerControls.tsx`
  - `src/components/timer/TimerContainer.tsx`

## 3. Instructions (세부 지침)

### Step 1: `TimerDisplay.tsx` 구현

다음 제공된 인터페이스와 유틸리티 함수를 정확히 사용하여 컴포넌트를 구현하세요.

```typescript
// 반드시 이 인터페이스를 컴포넌트의 Props로 사용할 것
export interface TimerDisplayProps {
  timeLeft: number; // 남은 초
  mode: "FOCUS" | "BREAK";
}

// 시간 포맷팅 유틸리티 (컴포넌트 내부 혹은 외부에 선언하여 사용할 것)
const formatTime = (seconds: number) => {
  const m = Math.floor(seconds / 60)
    .toString()
    .padStart(2, "0");
  const s = (seconds % 60).toString().padStart(2, "0");
  return `${m}:${s}`;
};
```

- **Typography:** `formatTime`을 거친 문자열을 화면에 렌더링할 때, `text-6xl` 이상의 크기와 `font-bold` 이상의 두께를 적용하세요.
- **Color Logic (Mode 조건부 스타일링):**
  - `mode === 'FOCUS'`일 때: 텍스트에 `text-red-500` 클래스 적용.
  - `mode === 'BREAK'`일 때: 텍스트에 `text-green-500` 클래스 적용.
- **Layout:** 숫자가 정중앙에 오도록 `flex`, `items-center`, `justify-center`를 활용하여 감싸는 컨테이너를 구성하세요.

### Step 2: `TimerControls.tsx` 구현

다음 제공된 인터페이스를 정확히 사용하여 컴포넌트를 구현하세요.

```typescript
// 반드시 이 인터페이스를 컴포넌트의 Props로 사용할 것
export interface TimerControlsProps {
  isRunning: boolean;
  onToggle: () => void;
  onReset: () => void;
}
```

- **UI Components:** `src/components/ui/button.tsx`의 Button 컴포넌트를 활용하여 2개의 버튼(Toggle, Reset)을 렌더링하세요.
- **Icons (`lucide-react` 사용):**
  - Toggle 버튼: `isRunning === true`이면 `Pause` 아이콘을, `false`이면 `Play` 아이콘을 렌더링하세요.
  - Reset 버튼: `Square` 또는 `RotateCcw` 아이콘을 렌더링하세요.
- **Interactions:** 버튼들에 `hover:` Tailwind prefix를 사용하여 hover 시 시각적으로 반응(색상 변경 또는 투명도 조절 등)하도록 트랜지션을 추가하세요.

### Step 3: `TimerContainer.tsx` 구현 (더미 데이터 조립)

하위 컴포넌트(`TimerDisplay`, `TimerControls`)를 감싸고 Props를 내려주는 부모 컨테이너를 구현하세요. Zustand는 사용하지 않으며, 아래의 더미 상태(Mock State) 코드를 삽입하여 렌더링을 테스트합니다.

```typescript
// TimerContainer 내부 Mock State (반드시 아래의 타입과 초기값을 준수할 것)
const = useState<number>(1500); // 25분
const = useState<boolean>(false);
const = useState<'FOCUS' | 'BREAK'>('FOCUS');
```

- **Behavior:**
  - `TimerControls`의 `onToggle` 호출 시 `setIsRunning`을 이용해 상태를 반전(`!isRunning`)시키기만 하세요.
  - 실제 시간이 줄어드는 로직(`useEffect` + `setInterval`)은 작성하지 마세요.

## 4. Constraints (제약 사항 및 금지 행동)

- `Zustand` store를 생성하거나 연결하지 마세요.
- 실제 타이머 로직(시간 차감)을 구현하지 마세요.
- UI 렌더링 시 Inline Style(`style={{ color: 'red' }}`)을 사용하지 말고, 반드시 Tailwind CSS 클래스명을 사용하세요.

## 5. Acceptance Criteria (Critic Agent 검증 체크리스트)

1. `TimerDisplay`, `TimerControls` 컴포넌트가 지시된 TypeScript `interface`를 완벽히 일치하게 선언하고 사용하였는가?
2. `TimerDisplay`에서 `mode` 값에 따라 `text-red-500`과 `text-green-500`이 조건부로 렌더링되도록 구현되었는가?
3. `TimerControls`에서 `lucide-react` 아이콘(Play, Pause 등)이 올바르게 조건부 렌더링되었는가?
4. `TimerContainer`에 더미 상태(Mock state)가 올바르게 선언되었으며, 타이머 차감 로직이나 Zustand 연동 코드가 존재하지 않는가?
5. `npm run lint` 실행 시 TypeScript 타입 에러 및 ESLint 에러가 없는가?
