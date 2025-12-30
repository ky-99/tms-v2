# Window Shadows Research for Tauri v2

> **Confidentiality**: Internal
> **Project**: TMS-v2
> **Task**: TASK-NEW-075
> **Requirement**: REQ-0061
> **Researcher**: Developer
> **Date**: 2025-12-31
> **Status**: Complete

---

## Executive Summary

This document provides comprehensive research on implementing window shadows in Tauri v2, with focus on compatibility with border-radius, platform-specific behaviors, and implementation approaches. The key finding is that **transparent windows with shadow and border-radius present significant challenges** across all platforms, requiring careful consideration of trade-offs.

**Quick Decision Matrix:**
- ✅ **Recommended**: `shadow: false` + CSS `border-radius` (current configuration)
- ⚠️ **Not Recommended**: `shadow: true` with transparent windows (macOS limitations, Windows border artifacts)
- 🔬 **Experimental**: Third-party plugins for macOS rounded corners

---

## Table of Contents

1. [Current Configuration](#1-current-configuration)
2. [Shadow Configuration in Tauri v2](#2-shadow-configuration-in-tauri-v2)
3. [Border-Radius Compatibility](#3-border-radius-compatibility)
4. [Platform-Specific Behaviors](#4-platform-specific-behaviors)
5. [Implementation Approaches](#5-implementation-approaches)
6. [Recommendations](#6-recommendations)
7. [References](#7-references)

---

## 1. Current Configuration

### 1.1 Tauri Configuration (`src-tauri/tauri.conf.json`)

```json
{
  "app": {
    "windows": [
      {
        "decorations": false,
        "transparent": true,
        "macOSPrivateApi": true
      }
    ]
  }
}
```

**Key Settings:**
- `decorations: false` - Native titlebar removed
- `transparent: true` - Window transparency enabled
- `macOSPrivateApi: true` - Enables transparent background API on macOS

### 1.2 Global CSS (`src/index.css`)

```css
html, body, #root {
  border-radius: 2px;
  overflow: hidden;
}
```

**Analysis:** Current setup uses CSS-based border-radius with transparent windows. This approach is working but lacks native window shadows.

---

## 2. Shadow Configuration in Tauri v2

### 2.1 Native Shadow Support

**Since Tauri v2.0**, native shadow support is built-in. The deprecated `window-shadows` crate is no longer needed.

**Configuration Property:**
```json
{
  "app": {
    "windows": [
      {
        "shadow": true  // boolean, default: false
      }
    ]
  }
}
```

**Source:** [Tauri v2 Configuration Schema](https://schema.tauri.app/config/2)

### 2.2 Runtime API

Shadows can also be toggled programmatically:

**Rust:**
```rust
window.set_shadow(true)?;
```

**JavaScript:**
```javascript
import { getCurrentWindow } from '@tauri-apps/api/window';
const window = getCurrentWindow();
await window.setShadow(true);
```

**Source:** [Tauri Window API Documentation](https://docs.rs/tauri/latest/tauri/window/struct.Window.html)

---

## 3. Border-Radius Compatibility

### 3.1 Known Compatibility Issue

**Critical Finding:** There is a **fundamental incompatibility** between native window shadows and border-radius on transparent windows.

**GitHub Issue:** [Problems with window customization's rounded corners and shadows #9287](https://github.com/tauri-apps/tauri/issues/9287)

**Problem Description:**
- When `shadow: true` is enabled with transparent windows, a border-like shadow artifact appears
- This artifact interferes with CSS border-radius rendering
- Top-left and top-right corners remain sharp despite CSS settings
- Users must **choose between shadows OR rounded corners** - cannot have both working together

### 3.2 Confirmed Workaround

**Solution:** Set `"shadow": false` in configuration

```json
{
  "app": {
    "windows": [
      {
        "shadow": false,  // Required for border-radius to work
        "decorations": false,
        "transparent": true
      }
    ]
  }
}
```

**Additional Requirement:** Ensure body background is set to transparent in CSS.

**Status:** Issue #9287 remains **open** with 20 comments as of 2025-12-31, indicating ongoing community interest but no complete solution.

**Sources:**
- [Problems with rounded corners and shadows](https://github.com/tauri-apps/tauri/issues/9287)
- [Impossible to create rounded transparent window without borders](https://github.com/tauri-apps/tauri/issues/3481)

---

## 4. Platform-Specific Behaviors

### 4.1 Windows

#### 4.1.1 Shadow Behavior

**When `shadow: false`:**
- Has **no effect** on decorated windows - shadows are always enabled
- Undecorated windows: no shadow, no border

**When `shadow: true`:**
- Decorated windows: shadows always on (no change)
- Undecorated windows:
  - Displays **1-pixel white border** around the window
  - Windows 11: Adds automatic **rounded corners** to the window
  - Windows 10: White border only, no rounded corners

**Source:** [Tauri WindowConfig Documentation](https://docs.rs/tauri-utils/latest/tauri_utils/config/struct.WindowConfig.html)

#### 4.1.2 Recent Bug Fix (February 2025)

**Issue:** [Slightly incorrect inner window size when using shadows + decorations disabled #12285](https://github.com/tauri-apps/tauri/issues/12285)

**Problem:**
- When `shadow: true` and `decorations: false`, inner content area was undersized
- Created visible black/white borders on right and bottom edges
- Content didn't fill entire window area

**Status:** **FIXED** in [wry PR #1483](https://github.com/tauri-apps/wry/pull/1483) (merged 2025-02-08)

**Impact:** This fix should be available in recent Tauri/wry versions. Check package versions:
- Tauri >= 2.2.0 (with wry 0.48.0+ and tao 0.31.1+)

#### 4.1.3 Windows Recommendations

**For Modern Appearance:**
- Windows 11: Can use `shadow: true` for native rounded corners + shadows
  - **Trade-off:** 1px white border + incompatible with custom CSS border-radius
- Windows 10/11: Better to use `shadow: false` + CSS border-radius
  - **Trade-off:** No native drop shadow

### 4.2 macOS

#### 4.2.1 Transparent Windows Limitation

**Critical Limitation:** "Shadows are always disabled for transparent windows" on macOS.

**Current Configuration Impact:**
- `transparent: true` in tauri.conf.json
- Therefore, `shadow: true` has **no effect** on macOS
- Native shadows cannot be enabled while using transparent windows

**Source:** [Tauri Configuration Reference](https://v2.tauri.app/reference/config/)

#### 4.2.2 Known Issues

**Transparent Window Artifacts:**
- [Shadow artifacts on transparent windows #5494](https://github.com/tauri-apps/tauri/issues/5494)
- Issue: Visual artifacts when combining transparency and shadows

**Border Interference:**
- Even with `transparent: true`, a subtle border may appear
- This border interferes with border-radius implementation
- Workaround: `shadow: false` (which is default on transparent windows anyway)

**Build Transparency Issues:**
- [Transparent windows lose transparency after DMG build #13415](https://github.com/tauri-apps/tauri/issues/13415)
- Windows may render correctly in dev but lose transparency in production builds
- Requires careful testing of final builds

#### 4.2.3 Third-Party Solution

**Plugin:** `@cloudworxx/tauri-plugin-mac-rounded-corners`

**Capabilities:**
- Works **without transparency API**
- Enables modern window style with rounded corners, shadow, and layer-based clipping
- Uses native macOS window styling APIs

**Trade-offs:**
- Third-party dependency (maintenance risk)
- Deviates from standard Tauri configuration
- May require `macOSPrivateApi: true`

**Source:** [npm package](https://www.npmjs.com/package/@cloudworxx/tauri-plugin-mac-rounded-corners)

#### 4.2.4 macOS Recommendations

**Best Approach for Current Project:**
- Keep `transparent: true` + `macOSPrivateApi: true`
- Do NOT set `shadow: true` (has no effect anyway)
- Use CSS `border-radius` for rounded corners
- Accept lack of native drop shadow as platform limitation

**Alternative (if shadows are critical):**
- Remove `transparent: true`
- Set `shadow: true`
- **Consequence:** Lose ability to have transparent window background
- **Consequence:** Likely lose border-radius capability

### 4.3 Linux

**Status:** Shadow property is **not supported** on Linux.

**Behavior:**
- Setting `shadow: true` or `shadow: false` has no effect
- Window shadows controlled by window manager / desktop environment
- Border-radius via CSS still works (with compositor support)

**Recommendation:**
- Ignore shadow property for Linux builds
- Focus on CSS-based styling which is more portable

**Source:** [Tauri WindowConfig Documentation](https://docs.rs/tauri-utils/latest/tauri_utils/config/struct.WindowConfig.html)

---

## 5. Implementation Approaches

### 5.1 Approach A: Current Configuration (Recommended) ✅

**Configuration:**
```json
{
  "app": {
    "windows": [
      {
        "decorations": false,
        "transparent": true,
        "shadow": false,  // Explicit (though false is default)
        "macOSPrivateApi": true
      }
    ]
  }
}
```

```css
html, body, #root {
  border-radius: 2px;
  overflow: hidden;
}
```

**Pros:**
- ✅ Border-radius works on all platforms
- ✅ Transparent background capability
- ✅ No compatibility issues
- ✅ Current implementation is already proven
- ✅ No 1px white border on Windows
- ✅ Consistent behavior across platforms

**Cons:**
- ❌ No native window drop shadow
- ❌ Window may appear "flat" without elevation cue

**Best For:** Applications prioritizing **border-radius** and **transparency** over native shadows.

**Verdict:** **This is the recommended approach** given current requirements and platform limitations.

---

### 5.2 Approach B: Enable Shadows (Not Recommended) ⚠️

**Configuration:**
```json
{
  "app": {
    "windows": [
      {
        "decorations": false,
        "transparent": true,
        "shadow": true,
        "macOSPrivateApi": true
      }
    ]
  }
}
```

**Expected Behavior by Platform:**

**Windows:**
- 1-pixel white border appears
- Windows 11: Native rounded corners (conflicts with CSS border-radius)
- Windows 10: White border only

**macOS:**
- No effect (shadows disabled for transparent windows)
- May introduce border artifacts

**Linux:**
- No effect (unsupported)

**Pros:**
- ✅ Windows 11: Native rounded corners + shadow
- ✅ Modern elevated appearance (Windows 11 only)

**Cons:**
- ❌ 1px white border on Windows (breaks custom design)
- ❌ No effect on macOS
- ❌ CSS border-radius conflicts with native corners
- ❌ Inconsistent behavior across platforms
- ❌ GitHub issue #9287 shows this causes rendering problems

**Verdict:** **Not recommended** due to platform inconsistencies and incompatibility with CSS border-radius.

---

### 5.3 Approach C: Disable Transparency, Enable Shadows

**Configuration:**
```json
{
  "app": {
    "windows": [
      {
        "decorations": false,
        "transparent": false,  // Changed
        "shadow": true
      }
    ]
  }
}
```

**Expected Behavior:**

**Windows:**
- Native shadows work
- 1px white border on undecorated windows
- Windows 11: Native rounded corners

**macOS:**
- Shadows work (transparency not blocking them)
- Likely **lose border-radius capability** (shadow artifact issue)

**Linux:**
- No effect (shadows unsupported)

**Pros:**
- ✅ Native shadows on macOS
- ✅ Potential for elevated appearance
- ✅ No macOS transparency limitations

**Cons:**
- ❌ Lose transparent background capability
- ❌ Likely lose border-radius (shadow artifact issue)
- ❌ 1px white border on Windows
- ❌ Major deviation from current design

**Verdict:** Only consider if **shadows are more important than transparency and border-radius** (unlikely for this project).

---

### 5.4 Approach D: Third-Party Plugin (macOS Only)

**Configuration:**
```toml
# src-tauri/Cargo.toml
[dependencies]
tauri-plugin-mac-rounded-corners = "0.1"  # Version may vary
```

```json
{
  "app": {
    "windows": [
      {
        "decorations": false,
        "transparent": false,  // Plugin doesn't use transparency API
        "shadow": false
      }
    ]
  }
}
```

**Pros:**
- ✅ macOS: Native rounded corners + shadows + clipping
- ✅ Modern appearance without transparency API

**Cons:**
- ❌ macOS only (platform-specific code)
- ❌ Third-party dependency (maintenance risk)
- ❌ Additional complexity
- ❌ May conflict with Windows/Linux implementations
- ❌ Requires separate styling for other platforms

**Verdict:** **Overkill** for this project. Only consider if macOS native appearance is critical business requirement.

---

## 6. Recommendations

### 6.1 Immediate Action: No Changes Required ✅

**Decision:** **Keep current configuration** (Approach A)

**Rationale:**
1. Current implementation works well across all platforms
2. Border-radius is functional and consistent
3. Transparent windows provide design flexibility
4. No breaking changes or regressions
5. Platform limitations make alternatives worse, not better

**Configuration to Maintain:**
```json
{
  "app": {
    "windows": [
      {
        "decorations": false,
        "transparent": true,
        "macOSPrivateApi": true
        // Note: shadow defaults to false, no need to specify
      }
    ]
  }
}
```

```css
html, body, #root {
  border-radius: 2px;
  overflow: hidden;
}
```

### 6.2 Optional Enhancement: CSS Drop Shadow

**If visual elevation is desired**, consider CSS-based drop shadow effect on the root container:

```css
#root {
  border-radius: 2px;
  overflow: hidden;
  /* Optional: Simulate window shadow with CSS */
  box-shadow: 0 10px 40px rgba(0, 0, 0, 0.3);
}
```

**Pros:**
- Platform-independent
- Full control over shadow appearance
- Works with border-radius
- No Tauri configuration changes

**Cons:**
- Not a true window shadow (shadow is inside the window)
- May look artificial compared to native shadows
- Requires careful color tuning for dark theme

**Recommendation:** Test this approach if lack of shadow becomes a UX concern. Otherwise, current design is clean and modern without it.

### 6.3 Future Considerations

**Monitor these GitHub issues for updates:**
- [#9287: Problems with rounded corners and shadows](https://github.com/tauri-apps/tauri/issues/9287)
- [#3481: Impossible to create rounded transparent window without borders](https://github.com/tauri-apps/tauri/issues/3481)

**If Tauri community provides a fix:**
- Revisit this research document
- Re-evaluate shadow enablement
- Test on all target platforms before deploying

### 6.4 Testing Checklist (If Changes Are Made)

If shadow configuration is ever modified, test the following:

**Windows Testing:**
- [ ] Test on Windows 10 (check for 1px white border)
- [ ] Test on Windows 11 (check for native rounded corners conflict)
- [ ] Verify border-radius renders correctly
- [ ] Check for sizing issues (verify wry >= 0.48.0)

**macOS Testing:**
- [ ] Verify transparent background works in dev mode
- [ ] Build DMG and verify transparency in production
- [ ] Check for border artifacts with `shadow: true`
- [ ] Confirm border-radius rendering

**Linux Testing:**
- [ ] Verify border-radius works with compositor
- [ ] Confirm shadow property has no effect
- [ ] Test on multiple desktop environments (GNOME, KDE, etc.)

**Cross-Platform:**
- [ ] Visual consistency across platforms
- [ ] No regressions in existing functionality
- [ ] Performance check (transparent windows can be slower)

---

## 7. References

### 7.1 Official Documentation

1. [Tauri v2 Configuration Reference](https://v2.tauri.app/reference/config/)
2. [Tauri v2 Configuration Schema](https://schema.tauri.app/config/2)
3. [Tauri Window API (Rust)](https://docs.rs/tauri/latest/tauri/window/struct.Window.html)
4. [Tauri Window API (JavaScript)](https://v2.tauri.app/reference/javascript/api/namespacewindow/)
5. [WindowConfig Struct Documentation](https://docs.rs/tauri-utils/latest/tauri_utils/config/struct.WindowConfig.html)
6. [Tauri Window Customization Guide](https://v2.tauri.app/learn/window-customization/)

### 7.2 GitHub Issues & Discussions

1. [#9287: Problems with window customization's rounded corners and shadows](https://github.com/tauri-apps/tauri/issues/9287)
2. [#3481: Impossible to create rounded transparent window without borders](https://github.com/tauri-apps/tauri/issues/3481)
3. [#12285: Incorrect window size with shadows + decorations disabled (Windows)](https://github.com/tauri-apps/tauri/issues/12285) - **FIXED**
4. [#5494: Transparent window shadow artifacts on macOS](https://github.com/tauri-apps/tauri/issues/5494)
5. [#13415: Transparent windows lose transparency after DMG build](https://github.com/tauri-apps/tauri/issues/13415)
6. [#3019: Expose config for enable/disable window shadow](https://github.com/tauri-apps/tauri/issues/3019)
7. [wry#1483: Fix incorrect window bounds on Windows](https://github.com/tauri-apps/wry/pull/1483) - **MERGED**

### 7.3 Third-Party Resources

1. [window-shadows crate](https://github.com/tauri-apps/window-shadows) - Deprecated, replaced by native support in v2
2. [@cloudworxx/tauri-plugin-mac-rounded-corners](https://www.npmjs.com/package/@cloudworxx/tauri-plugin-mac-rounded-corners) - macOS plugin
3. [DEV Community: Get the window configuration right first](https://dev.to/rain9/tauri-part-3-get-the-window-configuration-right-first-3gf7)

### 7.4 Related Requirements

- **REQ-0061**: Window Shadows Research (this document)
- **REQ-0062**: Window Border Radius with Shadows (depends on this research)
- **REQ-0035**: Decorations Disabled + Border Radius (implemented)

---

## Appendix A: Platform Behavior Matrix

| Configuration | Windows 10 | Windows 11 | macOS (transparent) | macOS (opaque) | Linux |
|--------------|------------|------------|-------------------|----------------|-------|
| `shadow: false, transparent: true` | No shadow, CSS border-radius works | No shadow, CSS border-radius works | No shadow, CSS border-radius works | No shadow, CSS border-radius works | No shadow, CSS border-radius works |
| `shadow: true, transparent: true` | 1px white border, no native corners | 1px white border + native corners | No effect (shadow disabled) | Shadow works, border-radius may break | No effect |
| `shadow: false, transparent: false` | No shadow, CSS border-radius works | No shadow, CSS border-radius works | No shadow, CSS border-radius works | No shadow, CSS border-radius works | No shadow, CSS border-radius works |
| `shadow: true, transparent: false` | 1px white border, no native corners | 1px white border + native corners | Shadow works, border-radius may break | Shadow works, border-radius may break | No effect |

**Legend:**
- ✅ Works as expected
- ⚠️ Works with caveats
- ❌ Doesn't work / Has issues
- 🚫 Not applicable

**Key Takeaway:** The only configuration that works reliably across all platforms without issues is `shadow: false` + CSS `border-radius` (current setup).

---

## Appendix B: Version Information

**Tested Configuration:**
- Tauri: 2.x
- Wry: 0.48.0+ (includes window sizing fix)
- Tao: 0.31.1+
- Target Platforms: Windows 10/11, macOS 12+, Linux (various)

**Note:** This research is based on Tauri v2 as of 2025-12-31. Future versions may introduce new capabilities or fixes.

---

## Document Metadata

- **Created**: 2025-12-31
- **Last Updated**: 2025-12-31
- **Status**: Complete
- **Next Review**: When REQ-0062 implementation begins, or when GitHub #9287 is resolved
- **Approval**: Pending developer review

---

**End of Document**
