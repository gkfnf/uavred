# Kanban UI Development Tasks

Based on the visual analysis of the provided MissionControl board design, the following tasks are required to implement the UI in `crates/dashboard_ui/`.

## 1. Layout & Grid Structure
- [ ] **Main Container**: Implement a horizontal scrolling container to hold the Kanban columns.
  - Use `h_flex()` with `w_full` and `h_full`.
  - Ensure background color matches the light off-white/gray background from the design (e.g., `bg_slate_50` or similar from theme).
- [ ] **Column Layout**: Implement the 5 specific columns:
  - **To Do**
  - **In Progress**
  - **In Review**
  - **Done**
  - **Cancelled**
- [ ] **Responsive/Fixed Width**: Determine if columns should have fixed width (e.g., `w_80`, `w_96`) or `flex_1` to fill available space. The design suggests equal width columns.

## 2. Component: Column Header
- [ ] **Refine `render_kanban_column_header`** (in `components.rs`):
  - **Layout**: `h_flex` with `justify_between` to separate the Title/Dot from the Add button.
  - **Status Indicator**: Add a colored circle dot before the text.
    - To Do: Black/Dark Grey
    - In Progress: Blue
    - In Review: Orange/Brown
    - Done: Green
    - Cancelled: Red
  - **Typography**: Bold, monospace or clean sans-serif font for the column title.
  - **Add Button**: Add a `+` icon button on the far right of the header.
  - **Styling**:
    - Light border bottom (dashed or solid light gray).
    - Padding (`p_2` or `p_4`).
    - Background matching the column or slightly distinct.

## 3. Component: Task Card
- [ ] **Refine `render_task_card`** (in `components.rs`):
  - **Container**: `div().bg_white().rounded_lg().shadow_sm().border_1().border_color(...)`.
  - **Header Section (Card)**:
    - **Title**: Bold text (e.g., "test", "A1_T0").
    - **Actions**:
      - "More" menu (`...`) icon at the top right.
      - (Conditional) "Delete/Cancel" icon (red circle X) near the menu.
  - **Body Section**:
    - Display the task description text (e.g., "UAVRed\_UI\_Tasks.md...").
    - Ensure text wrapping and proper line height.
    - Support for gray/muted text color for description.
  - **Spacing**: Ensure adequate padding inside the card (`p_4`) and margin between cards (`mb_4`).

## 4. Interactions & State
- [ ] **Drag and Drop**: Ensure cards are draggable between columns (referencing existing `mission_control.rs` logic if available, otherwise plan for implementation).
- [ ] **Card Actions**:
  - Implement the "..." menu click handler.
  - Implement the "Delete" (red X) click handler.
- [ ] **Add Task**: Connect the header `+` button to a "Create Task" action/modal.

## 5. Visual Polish
- [ ] **Borders & Separators**: Add vertical dividers between columns if strictly following the image (thin light lines).
- [ ] **Theme Alignment**: Ensure all colors (backgrounds, borders, text) use the `ui::theme` constants as per `AGENTS.md`.
