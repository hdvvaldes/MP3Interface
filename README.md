## Here is the project 2

# Plans: 

1. Rust - **backend**
2. Python - **middle part**
3. Tauri - **frontend**

┌─────────────────────────────────┐
│   Frontend (TypeScript/React)   │  ← UI, search bar, playlist, waveform
│   hosted in Tauri web view      │
├─────────────────────────────────┤
│     IPC / REST boundary         │  ← JSON messages, commands
├─────────────────────────────────┤
│   Rust Core                     │  ← tags, SQLite, audio engine
│   (id3 + sqlx + rodio + axum)   │
└─────────────────────────────────┘

