# Agent Guide for Auro

This document serves as the primary source of truth for AI agents and developers working on the Auro codebase.
It covers technical stack details, build procedures, code style conventions, and architectural patterns.

## 1. Project Overview & Tech Stack

Auro is a cross-platform desktop application bridging a React frontend with a Rust backend.

### Core Technologies
- **Frontend Framework:** React 19 + TypeScript
- **Build Tool:** Vite 7
- **Desktop Engine:** Tauri v2 (Rust)
- **Styling:** Tailwind CSS v4
- **UI Component Library:** shadcn/ui (Radix UI primitives)
- **Routing:** React Router DOM (using `HashRouter` for file-system compatibility)
- **State Management:** React Context + Local State
- **Form Handling:** React Hook Form + Zod validation
- **Icons:** Lucide React

## 2. Environment & Commands

The project uses `pnpm` for package management.

### Git Workflow & Version Control (IMPORTANT)
**Rules:**
1.  **NO direct commits to `main`:** Always use a feature branch.
2.  **Pull Requests:** All changes must go through a Pull Request (PR).
3.  **Branch Naming Convention:**
    - `feat/<feature-name>`: New features or significant enhancements.
    - `fix/<bug-name>`: Bug fixes or corrections.
    - `chore/<task-name>`: Maintenance, package upgrades, security fixes, or minor refactoring.
    - Example: `feat/add-funding-chart`, `fix/login-error-handling`.

### Setup & Installation
```bash
# Install frontend and tauri dependencies
pnpm install
```

### Development Workflow
| Command            | Description                            | Context                                            |
| :----------------- | :------------------------------------- | :------------------------------------------------- |
| `pnpm dev`         | Starts the Vite dev server (web mode). | Use for quick UI iteration without Rust APIs.      |
| `pnpm tauri dev`   | Starts the full desktop app.           | Use when working on Tauri APIs or native features. |
| `pnpm build`       | Type-checks and builds the frontend.   | Runs `tsc && vite build`.                          |
| `pnpm tauri build` | Builds the production executable.      | Creates the native binary.                         |

### Testing & Quality
| Command                  | Description                                                                                  |
| :----------------------- | :------------------------------------------------------------------------------------------- |
| `pnpm test`              | Runs all unit tests via Vitest.                                                              |
| `pnpm vitest run <path>` | Runs a specific test file. <br> Example: `pnpm vitest run src/components/ui/button.test.tsx` |
| `pnpm tauri check`       | Checks for Tauri-specific issues.                                                            |

**Agent Note:** Always run `pnpm build` after significant refactors to ensure type safety, as there is no separate `eslint` step.

## 3. Directory Structure & Architecture

```text
/
├── src/                  # React Frontend
│   ├── components/       # React Components
│   │   ├── ui/           # shadcn/ui primitives (Button, Input, etc.) - DO NOT MODIFY often
│   │   ├── common/       # Shared app-specific components (Sidebar, etc.)
│   │   ├── funding/      # Feature: Funding module
│   │   └── user-settings/# Feature: User Settings module
│   ├── lib/              # Utilities (cn, helpers)
│   ├── hooks/            # Custom React Hooks
│   ├── App.tsx           # Main Application Layout & Routing
│   └── main.tsx          # Entry point
├── src-tauri/            # Rust Backend (Tauri)
└── docs/                 # Documentation
```

### Architectural Patterns
1.  **Feature-Based Folders:** Group components by feature (e.g., `funding/`, `user-settings/`) rather than type.
2.  **UI Primitives:** Reusable, generic UI components live in `src/components/ui`. These are owned by `shadcn/ui`.
    *   **Rule:** Avoid modifying these directly. Extend them using `cva` variants if needed.
3.  **Absolute Imports:** Always use the `@/` alias.
    *   `@/components/ui/button` -> Resolves to `src/components/ui/button.tsx`
    *   `@/lib/utils` -> Resolves to `src/lib/utils.ts`

## 4. Code Style & Conventions

### TypeScript & Typing
- **Strict Mode:** Enabled. Do not use `any`.
- **Props:** Define component props using an `interface` or `type`.
- **HTML Attributes:** Use `React.ComponentProps<"element">` for flexible wrapper components.

```tsx
// ✅ Preferred Prop Definition
interface FundingCardProps extends React.ComponentProps<"div"> {
  amount: number;
  currency: string;
}
```

### Component Structure
- **Export:** Use named exports or default exports (project uses mixed, prefer `export default function` for pages/containers and named exports for UI).
- **Hooks:** Place hooks at the top of the component.
- **Handlers:** Prefix event handlers with `handle` (e.g., `handleSubmit`, `handleClick`).

### Styling (Tailwind CSS)
- **Utility First:** Use utility classes for everything.
- **Conditional Classes:** Use the `cn()` utility from `@/lib/utils` to merge classes safely.

```tsx
// ✅ Correct Styling Pattern
import { cn } from "@/lib/utils";

export function CustomCard({ className, children }: { className?: string, children: React.ReactNode }) {
  // Merges default styles with incoming className
  return (
    <div className={cn("p-4 rounded-lg bg-card text-card-foreground shadow", className)}>
      {children}
    </div>
  );
}
```

### Forms & Validation
- Use `react-hook-form` for state.
- Use `zod` for schema validation.
- Define schemas outside the component or in a separate file if complex.

### Database Migrations
- **Naming Convention:** Strictly follow `{number}_create_{type}_{name}.sql`.
    - `type`: Either `table` or `view`.
    - `name`: Descriptive name of the table or view.
    - Example: `5_create_table_fiat_ramp.sql`, `8_create_view_fiat_ramp_conversion.sql`.
- **Separation:** Keep table creation and view creation in separate migration files.

## 5. Testing Guidelines

- **Framework:** Vitest + React Testing Library.
- **Location:** Co-locate tests with the file they test (e.g., `MyComponent.test.tsx` next to `MyComponent.tsx`).
- **Mocking:** Use `vi` (from `vitest`) for mocking dependencies.

```tsx
// Example Test
import { render, screen } from "@testing-library/react";
import { describe, it, expect } from "vitest";
import MyComponent from "./MyComponent";

describe("MyComponent", () => {
  it("renders correctly", () => {
    render(<MyComponent />);
    expect(screen.getByText("Hello")).toBeInTheDocument();
  });
});
```

## 6. Common Pitfalls to Avoid

1.  **Relative Imports:** Do not use `../../` for root-level folders. Use `@/`.
2.  **Direct DOM Manipulation:** Always use React refs (`useRef`).
3.  **Tauri APIs:** Remember that `window` API might not be available in standard `pnpm dev` unless mocked. Wrap Tauri calls or check environment.
4.  **Icons:** Use `lucide-react` imports. Do not add FontAwesome or other icon libraries unless requested.
