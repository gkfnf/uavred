# Roadmap: UAVRed

**Project:** UAVRed - Desktop penetration testing tool for UAV ecosystems
**Core Value:** Security professionals can discover, analyze, and exploit vulnerabilities in UAV systems through an integrated desktop interface with AI-assisted analysis.
**Defined:** 2026-01-29
**Depth:** Comprehensive

## Overview

This roadmap delivers the v1 milestone of UAVRed, focusing on two major features: a redesigned Vulns panel with AI analysis capabilities and a complete Traffic panel for request/response inspection. The work is organized into 4 phases that build upon each other, starting with data layer improvements and ending with UI/UX polish.

## Phases

### Phase 1: Data Layer Foundation

**Goal:** Data layer supports full findings and traffic schema with proper repository patterns

**Dependencies:** None (foundation phase)

**Requirements:**
- DATA-01: Extend VulnStore to query full `findings` table schema
- DATA-02: Extend VulnStore to query `vulnerabilities` table for reference data
- DATA-05: Create repository methods for findings with AI analysis fields
- DATA-06: Ensure proper error handling for database operations
- DATA-07: Add database migrations for any missing tables/fields

**Success Criteria:**
1. VulnStore can query all fields from `findings` table including ai_confidence, ai_analysis, poc_code, mitre_techniques
2. VulnStore can query `vulnerabilities` table for CVE reference data
3. Database operations return proper error types with context
4. All required tables and fields exist in database schema
5. Repository pattern is consistent with existing TaskStore implementation

---

### Phase 2: Vulns Panel Redesign

**Goal:** Users can view and analyze vulnerabilities with AI-assisted insights in a 3-column layout

**Dependencies:** Phase 1 (requires data layer with AI fields)

**Requirements:**
- VULN-01: Redesign Vulns panel with 3-column layout matching Figma design
- VULN-02: Left column shows vulnerability list grouped by severity
- VULN-03: Left column displays CVE ID, severity badge, and AI detection indicator
- VULN-04: Middle column shows Details & PoC with full vulnerability description
- VULN-05: Middle column displays AI Security Analysis with confidence score
- VULN-06: Middle column shows AI-Generated PoC code with syntax highlighting
- VULN-07: Middle column displays MITRE ATT&CK techniques
- VULN-08: Right column shows CVE Database info with CVSS score
- VULN-09: Right column displays detection time, affected asset, and quick actions
- VULN-10: Connect to `findings` table with all fields
- VULN-11: Connect to `vulnerabilities` table for CVE reference data
- VULN-12: Real-time updates when findings are added/modified

**Success Criteria:**
1. User sees 3-column layout matching Figma design when opening Vulns panel
2. User can view vulnerability list grouped by severity with visual indicators
3. User can see AI detection indicators and confidence scores for AI-analyzed findings
4. User can view AI-generated PoC code with syntax highlighting in middle column
5. User can see MITRE ATT&CK techniques mapped to each vulnerability
6. User can view CVSS scores and CVE reference data in right column
7. Panel updates automatically when findings change in database

---

### Phase 3: Traffic Panel Implementation

**Goal:** Users can capture, inspect, and analyze network traffic with anomaly detection

**Dependencies:** Phase 1 (requires data layer), Phase 2 (sequential to manage complexity)

**Requirements:**
- DATA-03: Create TrafficStore for traffic data management
- DATA-04: Create repository methods for traffic CRUD operations
- TRAF-01: Implement Traffic panel with capture list and inspector layout
- TRAF-02: Top bar with TrafficQL search syntax highlighting
- TRAF-03: Capturing/Intercept toggle buttons in top bar
- TRAF-04: Left column shows captured traffic list with columns
- TRAF-05: Left column highlights anomalies with visual indicators
- TRAF-06: Middle section shows Request tab with headers and body
- TRAF-07: Middle section shows Response tab with headers and body
- TRAF-08: Right column shows Packet Info
- TRAF-09: Right column shows Anomaly Detection status
- TRAF-10: Right column provides Replay and Fuzz action buttons
- TRAF-11: Right column provides Export as cURL functionality
- TRAF-12: Right column shows Statistics
- TRAF-13: Right column shows Protocols breakdown
- TRAF-14: Connect to `traffic` table with all fields
- TRAF-15: Connect to `traffic_anomalies` table for anomaly details

**Success Criteria:**
1. User sees Traffic panel with 3-column layout when selected from sidebar
2. User can view captured traffic list with Time, Asset, Proto, Method, Path, Status columns
3. User can see anomaly indicators on suspicious traffic entries
4. User can inspect request and response headers/body in middle section
5. User can view packet info and anomaly detection status in right column
6. User can export traffic as cURL command
7. User can see traffic statistics and protocol breakdown in right column

---

### Phase 4: UI/UX Polish

**Goal:** Consistent, polished user experience with proper state handling across all panels

**Dependencies:** Phase 2, Phase 3 (applies to both redesigned panels)

**Requirements:**
- UI-01: Consistent theme usage across redesigned panels
- UI-02: Proper loading states while fetching data from SQLite
- UI-03: Empty state messages when no data exists
- UI-04: Error state handling for database connection issues

**Success Criteria:**
1. All colors, spacing, and typography use theme constants consistently
2. User sees loading indicators when data is being fetched
3. User sees helpful empty state messages when no vulnerabilities/traffic exist
4. User sees clear error messages if database connection fails
5. UI remains responsive during all database operations

---

## Progress

| Phase | Status | Requirements | Success Criteria Met |
|-------|--------|--------------|---------------------|
| 1 - Data Layer Foundation | Not Started | 5/5 | 0/5 |
| 2 - Vulns Panel Redesign | Not Started | 12/12 | 0/7 |
| 3 - Traffic Panel Implementation | Not Started | 16/16 | 0/7 |
| 4 - UI/UX Polish | Not Started | 4/4 | 0/5 |

**Coverage:** 37/37 v1 requirements mapped

## Notes

- Phase 1 must complete before Phases 2 and 3 can begin (data dependency)
- Phase 2 and 3 could theoretically be parallel but are sequenced for focus
- Phase 4 applies polish across both panels, so must come last
- Each phase delivers a working, testable increment

---
*Roadmap created: 2026-01-29*
