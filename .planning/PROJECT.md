# UAVRed

## What This Is

UAVRed is a desktop penetration testing tool for UAV (drone) ecosystems, built with Rust + GPUI. It provides autonomous agent-driven security testing with a modern, high-performance desktop UI.

## Core Value

Security professionals can discover, analyze, and exploit vulnerabilities in UAV systems through an integrated desktop interface with AI-assisted analysis.

## Requirements

### Validated

- ✓ Dashboard with mission control kanban and findings view — Phase 1
- ✓ Assets panel with network topology visualization — Phase 1
- ✓ Basic Vulns panel structure — Phase 1
- ✓ SQLite database integration for tasks — Phase 1
- ✓ Three-layer architecture (Presentation → State → Data) — Phase 1

### Active

- [ ] Redesign Vulns panel to match Figma design (3-column layout with AI analysis)
- [ ] Connect Vulns panel to full `findings` database table (AI confidence, PoC, MITRE ATT&CK)
- [ ] Implement Traffic panel with request/response inspection
- [ ] Connect Traffic panel to `traffic` and `traffic_anomalies` tables
- [ ] Fix data operations to use complete database schema

### Out of Scope

- Implementing actual scanner modules (network, protocol, firmware) — requires hardware testing
- Images panel redesign — deferred to v2
- Devices panel implementation — deferred to v2
- Settings panel implementation — deferred to v2
- Mobile app version — web-first approach for future

## Context

**Tech Stack**: Rust (2024 edition), GPUI (from Zed), gpui-component (60+ UI components), SQLite (via sqlez), Tokio async runtime

**Database Schema**: Comprehensive schema exists with tables for tasks, assets, vulnerabilities, findings (with AI fields), traffic, and workflows. See `database/schema.sql`.

**Design References**: Figma designs in `interface_pic/` showing 3-column layouts for Vulns and Traffic panels with AI analysis features.

**Current State**:
- Vulns panel has basic structure but doesn't match Figma design
- Traffic panel is stubbed ("Coming Soon")
- Data layer exists but panels don't use full schema
- AI analysis fields in database (ai_confidence, ai_analysis, poc_code) not displayed in UI

**Prior Work**: Codebase mapped in `.planning/codebase/` with architecture analysis following Zed editor patterns.

## Constraints

- **Tech Stack**: Must use existing GPUI + gpui-component architecture
- **Database**: SQLite only (no external database server)
- **Language**: Chinese comments OK, English code identifiers
- **Safety**: No unsafe code (workspace lint)

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Use Zed editor workspace pattern | Proven architecture for panel-based desktop apps | ✓ Good — clean separation of concerns |
| SQLite with sqlez wrapper | Native Rust async SQLite, proven in Zed | ✓ Good — works well for desktop |
| AI analysis in database schema | Future-proof for AI agent integration | — Pending — schema ready but UI not using it |
| GPUI immediate-mode UI | GPU accelerated, responsive | ✓ Good — fast UI updates |

---
*Last updated: 2026-01-29 after codebase mapping*
