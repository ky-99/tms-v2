# Task Breakdown: UI Enhancement Phase 4
> Created: 2025-12-30
> Requirements: REQ-0054 through REQ-0062
> Status: Ready for Implementation

---

## Summary

This document breaks down the new UI enhancement requirements (REQ-0054~REQ-0062) into implementable tasks.

### Requirements Added:
1. **Modal Design Improvements** (REQ-0054~0059): English localization, standardized inputs, Kobalte components, enhanced tag selector
2. **Tag Filter Upgrade** (REQ-0060): Replace simple checkboxes with Kobalte components
3. **Window Styling** (REQ-0061~0062): Research and implement window shadows with border-radius

---

## Task Breakdown

### Group 1: Modal Localization & Standardization
**Priority**: P1 (SHOULD)
**Estimated Complexity**: Low-Medium

#### TASK-NEW-068: Modal Labels English Localization
- **Maps to**: REQ-0054
- **Description**: Change all modal labels from Japanese to English
- **Component**: TaskPage (Dialog component)
- **DoD**:
  - [ ] All labels (title, description, tags, parent, status) are in English
  - [ ] Modal displays correctly with English text
  - [ ] No layout issues with English labels
- **Files to modify**:
  - `src/pages/TaskPage.tsx`

---

#### TASK-NEW-069: Standardize Input Field Design
- **Maps to**: REQ-0055
- **Description**: Ensure consistent styling across all input fields
- **Component**: Input, Textarea components
- **Depends on**: TASK-NEW-068
- **DoD**:
  - [ ] All input fields (title, description, tags) have same border style
  - [ ] Padding is consistent across inputs
  - [ ] Focus states are uniform
  - [ ] Typography (font-size, line-height) is consistent
- **Files to modify**:
  - `src/components/Input.tsx`
  - `src/components/Textarea.tsx` (if exists)
  - `src/pages/TaskPage.tsx`

---

### Group 2: Kobalte Component Integration
**Priority**: P1 (SHOULD)
**Estimated Complexity**: Medium-High

#### TASK-NEW-070: Kobalte Parent Selector Implementation
- **Maps to**: REQ-0056
- **Description**: Replace simple select with Kobalte Select for parent task selection
- **Component**: TaskPage (Dialog), Kobalte Select
- **Depends on**: TASK-NEW-069
- **DoD**:
  - [ ] Kobalte Select component integrated
  - [ ] Shows list of available parent tasks
  - [ ] Search/filter functionality works
  - [ ] Empty state displayed when no parents available
  - [ ] Matches standardized input design
- **Files to modify**:
  - `src/pages/TaskPage.tsx`
- **New dependencies**: `@kobalte/core` Select component

---

#### TASK-NEW-071: Tag Selector - Show All Candidates
- **Maps to**: REQ-0057
- **Description**: Display all available tags at once for easy selection
- **Component**: TagInput
- **Depends on**: TASK-NEW-069
- **DoD**:
  - [ ] All tags displayed in selector (not just selected)
  - [ ] User can see complete tag list
  - [ ] Can select/deselect tags easily
  - [ ] Empty state message when no tags exist
  - [ ] Visual indication of selected vs unselected tags
- **Files to modify**:
  - `src/components/TagInput.tsx`
- **API used**: `listTags`

---

#### TASK-NEW-072: Tag Selector - Inline Tag Creation
- **Maps to**: REQ-0058
- **Description**: Allow creating new tags within task modal
- **Component**: TagInput
- **Depends on**: TASK-NEW-071
- **DoD**:
  - [ ] "Create Tag" button visible in tag selector
  - [ ] Inline input for new tag name
  - [ ] Create/Cancel buttons match modal button design
  - [ ] Validation: empty names rejected
  - [ ] Validation: duplicate names rejected
  - [ ] New tag immediately available for selection
  - [ ] Cancel discards creation
- **Files to modify**:
  - `src/components/TagInput.tsx`
- **API used**: `createTag`

---

### Group 3: UX Improvements
**Priority**: P2 (SHOULD)
**Estimated Complexity**: Low

#### TASK-NEW-073: Auto-focus Title on Create Modal
- **Maps to**: REQ-0059
- **Description**: Automatically focus title field when create modal opens
- **Component**: TaskPage (Dialog)
- **DoD**:
  - [ ] Create modal auto-focuses title input
  - [ ] Edit modal does NOT auto-focus
  - [ ] Cursor ready for immediate typing in create mode
  - [ ] No auto-focus when editing existing task
- **Files to modify**:
  - `src/pages/TaskPage.tsx`
- **Implementation**: Use Solid.js `onMount` or `createEffect` with mode check

---

### Group 4: Tag Filter Redesign
**Priority**: P1 (SHOULD)
**Estimated Complexity**: Medium

#### TASK-NEW-074: Kobalte Tag Filter Component
- **Maps to**: REQ-0060
- **Description**: Replace simple checkbox with Kobalte-based design
- **Component**: TaskPool, TagFilter
- **DoD**:
  - [ ] Old checkbox design removed
  - [ ] Kobalte component integrated (Popover + Checkbox group recommended)
  - [ ] Cleaner, more modern visual design
  - [ ] Better UX than current implementation
  - [ ] All filtering functionality preserved
  - [ ] Responsive design
- **Files to modify**:
  - `src/components/TagFilter.tsx` (or create new)
  - `src/components/TaskPool.tsx`
- **New dependencies**: `@kobalte/core` Popover, Checkbox components

---

### Group 5: Window Styling Research & Implementation
**Priority**: P2 (COULD)
**Estimated Complexity**: Medium (Research) + Low (Implementation)

#### TASK-NEW-075: Window Shadows Research
- **Maps to**: REQ-0061
- **Description**: Research Tauri window shadows capabilities
- **DoD**:
  - [ ] Documentation created covering:
    - How to enable window shadows in Tauri v2
    - Compatibility with border-radius
    - Platform-specific behaviors (macOS/Windows/Linux)
    - Implementation approach
  - [ ] Alternative approaches documented if shadows not supported
  - [ ] Recommendations made for implementation
- **Output**: Research document (Markdown file in ai-vault)
- **Resources**:
  - Tauri v2 documentation
  - Tauri GitHub issues/discussions
  - Platform-specific window APIs

---

#### TASK-NEW-076: Window Border Radius with Shadows Implementation
- **Maps to**: REQ-0062
- **Description**: Implement window shadows based on research findings
- **Component**: tauri.conf.json, global CSS
- **Depends on**: TASK-NEW-075
- **DoD**:
  - [ ] Window shadows enabled (if supported)
  - [ ] Border-radius maintained
  - [ ] Modern, elevated appearance achieved
  - [ ] Platform-specific behavior documented
  - [ ] Fallback to border-radius only if shadows not supported
- **Files to modify**:
  - `src-tauri/tauri.conf.json`
  - `src/index.css` or equivalent global styles
- **Platform testing**: macOS, Windows (if available)

---

## Implementation Order Recommendation

### Phase 1: Foundation (P1 tasks)
1. **TASK-NEW-068**: Modal Labels English (quick win)
2. **TASK-NEW-069**: Standardize Input Design (foundation for others)
3. **TASK-NEW-073**: Auto-focus Title (quick win, good UX improvement)

### Phase 2: Tag Enhancements (P1 tasks)
4. **TASK-NEW-071**: Tag Selector - Show All Candidates
5. **TASK-NEW-072**: Tag Selector - Inline Creation
6. **TASK-NEW-074**: Kobalte Tag Filter

### Phase 3: Advanced Components (P1 task)
7. **TASK-NEW-070**: Kobalte Parent Selector

### Phase 4: Polish (P2 tasks)
8. **TASK-NEW-075**: Window Shadows Research
9. **TASK-NEW-076**: Window Shadows Implementation

---

## Dependencies Summary

```
TASK-NEW-068 (English Labels)
  └─> TASK-NEW-069 (Standardized Inputs)
        ├─> TASK-NEW-070 (Kobalte Parent Selector)
        └─> TASK-NEW-071 (Tag Show All)
              └─> TASK-NEW-072 (Tag Inline Creation)

TASK-NEW-073 (Auto-focus) - Independent

TASK-NEW-074 (Tag Filter) - Independent

TASK-NEW-075 (Research)
  └─> TASK-NEW-076 (Shadows Implementation)
```

---

## Risks & Considerations

### Technical Risks
1. **Kobalte Learning Curve**: Team needs to learn Kobalte component APIs
2. **Tag Selector Complexity**: Showing all tags + inline creation may be complex UI
3. **Window Shadows Compatibility**: May not work on all platforms or with border-radius

### Mitigation
1. Review Kobalte documentation before starting TASK-NEW-070, 072, 074
2. Design tag selector UI mockup before implementation
3. Research thoroughly (TASK-NEW-075) before attempting implementation

---

## Next Steps

1. ✅ Requirements documented in `requirements.md`
2. ⏭️ Review and approve this task breakdown
3. ⏭️ Add tasks to `tasks.md`
4. ⏭️ Begin implementation starting with Phase 1

---

## Notes

- REQ-0042 and REQ-0043 marked as **Deprecated** (replaced by REQ-0054~0060)
- All new requirements follow "1 REQ = 1 Acceptance Criterion" rule
- Task priorities align with requirement priorities (SHOULD → P1, COULD → P2)
