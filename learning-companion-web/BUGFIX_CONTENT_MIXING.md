# Content Mixing Bug Fix

## Date
2026-01-31

## Issue Description
**Bug**: Python and Go learning modules were displaying Rust content when viewed through the web interface.

**Root Cause**: The backend API endpoint `/api/modules/{module_id}/content/{type}` did not include language information. Since all module directories across languages use the same naming convention (e.g., `module-01-basics`), the backend's search loop that checks directories in order `["rust", "python", "go"]` would always find and return Rust content first.

## Solution
Added language as a path parameter to all module content endpoints:

- **Old**: `/api/modules/{module_id}/content/{type}`
- **New**: `/api/modules/{language}/{module_id}/content/{type}`

This change ensures the backend directly accesses the correct language directory without searching.

## Changes Made

### Backend Changes (server/src/)

#### 1. main.rs - Route Definitions
Updated three route patterns to include language parameter:
```rust
// Before:
.route("/api/modules/:id/content/:type", get(api::get_module_content))
.route("/api/modules/:id/examples", get(api::list_examples))
.route("/api/modules/:id/examples/:filename", get(api::get_example_content))

// After:
.route("/api/modules/:language/:id/content/:type", get(api::get_module_content))
.route("/api/modules/:language/:id/examples", get(api::list_examples))
.route("/api/modules/:language/:id/examples/:filename", get(api::get_example_content))
```

#### 2. api.rs - Handler Functions

**get_module_content**:
- Changed signature from `Path((module_id, content_type))` to `Path((language, module_id, content_type))`
- Removed directory search loop
- Directly constructs path: `project_path.join(&language).join(&module_id)`

**list_examples**:
- Changed signature from `Path(module_id)` to `Path((language, module_id))`
- Removed directory search loop
- Updated file extension check to support `.rs`, `.py`, and `.go` files
- Directly constructs path: `project_path.join(&language).join(&module_id)`

**get_example_content**:
- Changed signature from `Path((module_id, filename))` to `Path((language, module_id, filename))`
- Removed directory search loop
- Directly constructs path: `project_path.join(&language).join(&module_id)`

### Frontend Changes (client/src/)

#### 3. api.ts - API Client Methods

Updated method signatures to accept `LearningModule` object instead of just `moduleId` string:
```typescript
// Before:
getContent: async (moduleId: string, contentType: string) => { ... }
listExamples: async (moduleId: string) => { ... }
getExampleContent: async (moduleId: string, filename: string) => { ... }

// After:
getContent: async (module: LearningModule, contentType: string) => { ... }
listExamples: async (module: LearningModule) => { ... }
getExampleContent: async (module: LearningModule, filename: string) => { ... }
```

Updated API call URLs to include language:
```typescript
// Before:
`/modules/${moduleId}/content/${contentType}`

// After:
`/modules/${module.language}/${module.id}/content/${contentType}`
```

#### 4. components/ContentViewer.tsx - Component Usage

Updated three API calls to pass `module` object instead of `module.id`:
- Line 44: `modulesApi.listExamples(module)` (was `module.id`)
- Line 49: `modulesApi.getContent(module, contentType)` (was `module.id, contentType`)
- Line 67: `modulesApi.getExampleContent(module, filename)` (was `module.id, filename`)

## Data Flow

### Before (Broken):
```
Frontend: module.id = "module-01-basics"
         ↓
API Call: GET /api/modules/module-01-basics/content/readme
         ↓
Backend: Searches directories in order:
  1. rust/module-01-basics ✅ FOUND → Returns Rust README
  2. python/module-01-basics ❌ Not searched (found in step 1)
  3. go/module-01-basics ❌ Not searched
```

### After (Fixed):
```
Frontend: module.language = "python", module.id = "module-01-basics"
         ↓
API Call: GET /api/modules/python/module-01-basics/content/readme
         ↓
Backend: Directly accesses python/module-01-basics → Returns Python README
```

## Testing

### Backend Compilation
✅ `cargo check` passed successfully (only minor unused variable warnings)

### Manual Testing Required
1. Start backend: `cd server && cargo run`
2. Start frontend: `cd client && npm run dev`
3. Open browser to `http://localhost:5173`
4. Test each language:
   - Select Python → View module-01-basics README → Should show Python content
   - Select Go → View module-01-basics README → Should show Go content
   - Select Rust → View module-01-basics README → Should show Rust content

### Expected Results
- Python modules should display Python content (README, examples, exercises)
- Go modules should display Go content
- Rust modules should display Rust content
- No content mixing between languages

## Impact
- **Breaking Change**: Yes - API endpoint paths have changed
- **Migration**: Frontend has been updated to use new paths
- **Backward Compatibility**: Old endpoints no longer work (will need to redeploy both frontend and backend together)

## Related Files
- `learning-companion-web/server/src/main.rs`
- `learning-companion-web/server/src/api.rs`
- `learning-companion-web/client/src/api.ts`
- `learning-companion-web/client/src/components/ContentViewer.tsx`
