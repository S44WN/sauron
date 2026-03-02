 ┌──────────────────────────────┐
 │        Command Center        │
 │   (3D Map + Alerts + UI)     │
 │   React + Three.js + Tauri   │
 └─────────────▲────────────────┘
               │ WebSockets (JSON)
 ┌─────────────┴────────────────┐
 │        Backend / Control     │
 │  Rules, Zones, Events, Auth  │
 │   Rust / Go + PostgreSQL     │
 └─────────────▲────────────────┘
               │ Telemetry (Low BW)
 ┌─────────────┴────────────────┐
 │        Edge Node (THIS)      │
 │  Camera → AI → World Coords  │
 │  Rust + OpenCV + AI Runtime  │
 └─────────────▲────────────────┘
               │ RTSP / Sensors
 ┌─────────────┴────────────────┐
 │     Cameras / Drones / IoT   │
 └──────────────────────────────┘
