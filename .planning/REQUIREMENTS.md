# Requirements: UAVRed

**Defined:** 2026-01-29
**Core Value:** Security professionals can discover, analyze, and exploit vulnerabilities in UAV systems through an integrated desktop interface with AI-assisted analysis.

## v1 Requirements

### Vulns Panel Redesign

- [ ] **VULN-01**: Redesign Vulns panel with 3-column layout matching Figma design
- [ ] **VULN-02**: Left column shows vulnerability list grouped by severity (Critical, High, Medium, Low)
- [ ] **VULN-03**: Left column displays CVE ID, severity badge, and AI detection indicator per vulnerability
- [ ] **VULN-04**: Middle column shows Details & PoC with full vulnerability description
- [ ] **VULN-05**: Middle column displays AI Security Analysis with confidence score, exploitability, and potential impact
- [ ] **VULN-06**: Middle column shows AI-Generated PoC code with syntax highlighting
- [ ] **VULN-07**: Middle column displays MITRE ATT&CK techniques
- [ ] **VULN-08**: Right column shows CVE Database info with CVSS score
- [ ] **VULN-09**: Right column displays detection time, affected asset, and quick actions
- [ ] **VULN-10**: Connect to `findings` table with all fields (ai_confidence, ai_analysis, poc_code, mitre_techniques)
- [ ] **VULN-11**: Connect to `vulnerabilities` table for CVE reference data
- [ ] **VULN-12**: Real-time updates when findings are added/modified in database

### Traffic Panel Implementation

- [ ] **TRAF-01**: Implement Traffic panel with capture list and inspector layout
- [ ] **TRAF-02**: Top bar with TrafficQL search syntax highlighting
- [ ] **TRAF-03**: Capturing/Intercept toggle buttons in top bar
- [ ] **TRAF-04**: Left column shows captured traffic list with columns: #, Time, Asset, Proto, Method, Path, Status, Size, Duration
- [ ] **TRAF-05**: Left column highlights anomalies with visual indicators
- [ ] **TRAF-06**: Middle section shows Request tab with headers and body
- [ ] **TRAF-07**: Middle section shows Response tab with headers and body
- [ ] **TRAF-08**: Right column shows Packet Info (ID, Size, Time)
- [ ] **TRAF-09**: Right column shows Anomaly Detection status with type
- [ ] **TRAF-10**: Right column provides Replay and Fuzz action buttons
- [ ] **TRAF-11**: Right column provides Export as cURL functionality
- [ ] **TRAF-12**: Right column shows Statistics (Total, Anomalies, Success %, Avg Time)
- [ ] **TRAF-13**: Right column shows Protocols breakdown
- [ ] **TRAF-14**: Connect to `traffic` table with all fields
- [ ] **TRAF-15**: Connect to `traffic_anomalies` table for anomaly details

### Data Layer Improvements

- [ ] **DATA-01**: Extend VulnStore to query full `findings` table schema
- [ ] **DATA-02**: Extend VulnStore to query `vulnerabilities` table for reference data
- [ ] **DATA-03**: Create TrafficStore for traffic data management
- [ ] **DATA-04**: Create repository methods for traffic CRUD operations
- [ ] **DATA-05**: Create repository methods for findings with AI analysis fields
- [ ] **DATA-06**: Ensure proper error handling for database operations
- [ ] **DATA-07**: Add database migrations for any missing tables/fields

### UI/UX Improvements

- [ ] **UI-01**: Consistent theme usage across redesigned panels
- [ ] **UI-02**: Proper loading states while fetching data from SQLite
- [ ] **UI-03**: Empty state messages when no data exists
- [ ] **UI-04**: Error state handling for database connection issues

## v2 Requirements

### Images Panel

- **IMG-01**: Implement Images panel for firmware image analysis
- **IMG-02**: Display uploaded firmware images with metadata
- **IMG-03**: Show extraction results and file tree

### Devices Panel

- **DEV-01**: Implement Devices panel for hardware device management
- **DEV-02**: Display connected hardware devices
- **DEV-03**: Show device configuration and status

### Settings Panel

- **SET-01**: Implement Settings panel for application configuration
- **SET-02**: Database connection settings
- **SET-03**: Theme and appearance settings

## Out of Scope

| Feature | Reason |
|---------|--------|
| Actual scanner implementations | Requires hardware testing environment not available |
| Real-time network packet capture | Requires root/admin privileges and platform-specific code |
| AI model training/execution | Use pre-trained models via API or local inference only |
| Multi-user support | Single-user desktop application |
| Cloud synchronization | Local-first architecture |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| VULN-01 | Phase 1 | Pending |
| VULN-02 | Phase 1 | Pending |
| VULN-03 | Phase 1 | Pending |
| VULN-04 | Phase 1 | Pending |
| VULN-05 | Phase 1 | Pending |
| VULN-06 | Phase 1 | Pending |
| VULN-07 | Phase 1 | Pending |
| VULN-08 | Phase 1 | Pending |
| VULN-09 | Phase 1 | Pending |
| VULN-10 | Phase 1 | Pending |
| VULN-11 | Phase 1 | Pending |
| VULN-12 | Phase 1 | Pending |
| TRAF-01 | Phase 2 | Pending |
| TRAF-02 | Phase 2 | Pending |
| TRAF-03 | Phase 2 | Pending |
| TRAF-04 | Phase 2 | Pending |
| TRAF-05 | Phase 2 | Pending |
| TRAF-06 | Phase 2 | Pending |
| TRAF-07 | Phase 2 | Pending |
| TRAF-08 | Phase 2 | Pending |
| TRAF-09 | Phase 2 | Pending |
| TRAF-10 | Phase 2 | Pending |
| TRAF-11 | Phase 2 | Pending |
| TRAF-12 | Phase 2 | Pending |
| TRAF-13 | Phase 2 | Pending |
| TRAF-14 | Phase 2 | Pending |
| TRAF-15 | Phase 2 | Pending |
| DATA-01 | Phase 1 | Pending |
| DATA-02 | Phase 1 | Pending |
| DATA-03 | Phase 2 | Pending |
| DATA-04 | Phase 2 | Pending |
| DATA-05 | Phase 1 | Pending |
| DATA-06 | Phase 1-2 | Pending |
| DATA-07 | Phase 1 | Pending |
| UI-01 | Phase 1-2 | Pending |
| UI-02 | Phase 1-2 | Pending |
| UI-03 | Phase 1-2 | Pending |
| UI-04 | Phase 1-2 | Pending |

**Coverage:**
- v1 requirements: 40 total
- Mapped to phases: 40
- Unmapped: 0 ✓

---
*Requirements defined: 2026-01-29*
*Last updated: 2026-01-29 after initial definition*
