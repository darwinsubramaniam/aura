# Aura

Aura is a privacy-focused crypto tracking app built with Tauri, React, and TypeScript.
Still in active development.

## Features

### 📊 Dashboard & Analytics
- **Interactive Charts**: Visualise your portfolio and funding history using powerful Apache ECharts.
- **Summary Cards**: At-a-glance view of key metrics and financial status.

### 💰 Funding Management
- **Comprehensive Tracking**: Manage and track all your funding sources and history.
- **Bulk Import**: Support for importing funding records via Excel/CSV files.
- **CRUD Operations**: Full Create, Read, Update, Delete capabilities for funding entries.
- **Advanced Filtering**: Filter history by date ranges and other criteria.

### 💱 Fiat Operations
- **Fiat Ramp Tracking**: Dedicated module to manage and track fiat on/off ramp transactions.
- **Exchange Rates**: Integrated exchange rate tracking with support for historical data.
- **Multi-Currency**: Support for multiple fiat currencies.

### 🛠️ Core Capabilities
- **Cross-Platform**: Optimized for Desktop (macOS, Windows, Linux) and Mobile (Android, iOS).
- **Privacy First**: All data is stored locally on your device using SQLite. No external servers hold your financial data.
- **User Settings**: Customizable application preferences and theming (Dark/Light mode).

## Tech Stack

### Frontend
- **Framework**: React 19
- **Build Tool**: Vite 7
- **Styling**: TailwindCSS v4, Radix UI (shadcn/ui), Lucide Icons
- **Charting**: Apache ECharts
- **State & Routing**: React Router v7, React Hook Form, Zod
- **Language**: TypeScript

### Backend
- **Core**: Tauri v2 (Rust)
- **Database**: SQLite (via SQLx)
- **Architecture**: Local-first, offline-capable

## Prerequisites

- **Node.js**: v20+ recommended
- **Rust**: Stable release
- **Package Manager**: pnpm
- **Mobile Dev**: Android Studio (Android), Xcode (iOS - macOS only)

## Getting Started

### Installation

Clone the repo and install dependencies:

```bash
pnpm install
```

### Development

**Desktop:**

```bash
pnpm run tauri dev
```

**Android:**

```bash
pnpm run tauri android dev
```

**iOS:**

```bash
pnpm run tauri ios dev
```

|                                     Desktop                                     |                                     Android                                     |                                   iOS                                   |
| :-----------------------------------------------------------------------------: | :-----------------------------------------------------------------------------: | :---------------------------------------------------------------------: |
| <img src="docs/screenshots/desktop.png" alt="Desktop Screenshot" width="500" /> | <img src="docs/screenshots/android.png" alt="Android Screenshot" width="250" /> | <img src="docs/screenshots/ios.png" alt="iOS Screenshot" width="250" /> |

## License

[MIT](LICENSE)
