# Gemini Translation Task Specification

**Date**: 2025-10-27  
**Task ID**: translation-vietnamese-to-english-2025-10-27  
**Priority**: P1 HIGH  
**Estimated Duration**: 3 hours  
**Assigned AA**: Gemini (Google AI)  
**Status**: Ready for claim

---

## 🎯 TASK OVERVIEW

### Objective
Translate Vietnamese content in 5 documentation files to English while preserving technical accuracy and meaning.

### Context
- **Language Policy**: ALL documentation MUST be in English (`.agents/workflows/LANGUAGE_POLICY.md`)
- **Violations Found**: 5 files created this session contain Vietnamese content
- **User Requirement**: Vietnamese for user communication, English for all project documentation
- **Reason**: Enable international collaboration, professional standards, multi-AA compatibility

### Success Criteria
- ✅ 100% of documentation files in English
- ✅ Technical accuracy preserved (terms, concepts, code references)
- ✅ Clear, professional English (no machine translation artifacts)
- ✅ No Vietnamese characters in `.md` or `.yml` files
- ✅ All files pass language policy compliance check
- ✅ Format and structure preserved (YAML/Markdown syntax intact)

---

## 📋 FILES TO TRANSLATE

### Total: 5 files (~2,000 words Vietnamese content)

| File | Path | Vietnamese % | Est. Time | Priority |
|------|------|--------------|-----------|----------|
| 1 | `.agents/workflows/CODEX_EXPECTED_BEHAVIOR_SUMMARY.md` | 80% | 45 mins | HIGH |
| 2 | `.agents/lessons_learned/META_LEARNING_WHEN_TO_CREATE_LESSONS.md` | 60% | 45 mins | HIGH |
| 3 | `.agents/lessons_learned/REALITY_CHECK_PROVEN_VS_PROPOSED.md` | 40% | 30 mins | MEDIUM |
| 4 | `.agents/lessons_learned/CURSOR_PROTOCOL_VIOLATION_META_LESSON_2025-10-27.md` | 50% | 30 mins | MEDIUM |
| 5 | `.agents/workflows/SESSION_SUMMARY_2025-10-27.md` | 10% | 15 mins | LOW |

**Total Estimated Time**: 2 hours 45 minutes

---

## 🔧 TRANSLATION GUIDELINES

### Technical Accuracy
```yaml
DO:
  ✅ Keep technical terms in English (e.g., "branch", "commit", "merge", "pull request")
  ✅ Keep code snippets unchanged
  ✅ Keep file paths unchanged
  ✅ Keep URL references unchanged
  ✅ Keep acronyms in English (e.g., "AA", "CI", "PR", "GUI")

DON'T:
  ❌ Translate technical terms to English equivalents of Vietnamese
  ❌ Modify code or configuration examples
  ❌ Change YAML structure or keys
  ❌ Alter Markdown formatting
```

### Professional English
```yaml
Style:
  - Clear and concise (avoid verbosity)
  - Professional tone (not casual)
  - Active voice preferred
  - Present tense for current state, past tense for completed actions

Common Patterns:
  Vietnamese: "Tôi đã tạo file X"
  English: "Created file X" (professional, concise)
  
  Vietnamese: "Bạn có thể làm..."
  English: "You can..." or "To do this..." (clear, direct)
  
  Vietnamese: "Điều này rất quan trọng"
  English: "This is critical" or "This is important" (professional)
```

### Format Preservation
```yaml
Markdown Files (.md):
  - Preserve headers (# ## ###)
  - Preserve lists (- bullets, 1. numbered)
  - Preserve code blocks (```language)
  - Preserve tables (| column |)
  - Preserve emphasis (**bold**, *italic*)

YAML Files (.yml):
  - Preserve key names (don't translate keys)
  - Translate only string values
  - Preserve indentation exactly
  - Preserve comments structure
```

---

## 📝 TRANSLATION WORKFLOW

### Step 1: Claim Task (5 mins)
```yaml
Action: Post in Issue #56
Message: "I claim translation task: 5 files Vietnamese → English. ETA: 3 hours."
Wait: For acknowledgment or proceed after 5 mins
```

### Step 2: Setup (5 mins)
```yaml
1. Read LANGUAGE_POLICY.md (understand requirements)
2. Read this spec completely
3. Clone/pull latest from feature branch
4. Create checklist for 5 files
```

### Step 3: Translate Files (2h 45m)
```yaml
For each file:
  1. Open file and identify Vietnamese sections
  2. Translate section by section (not entire file at once)
  3. Verify technical terms preserved
  4. Check YAML/Markdown syntax intact
  5. Review translation for professional quality
  6. Mark file complete in checklist
```

### Step 4: Quality Assurance (15 mins)
```yaml
Self-Check:
  □ All 5 files translated
  □ No Vietnamese characters remain (use: grep -r "[àáảãạâầấẩẫậăằắẳẵặèéẻẽẹêềếểễệìíỉĩịòóỏõọôồốổỗộơờớởỡợùúủũụưừứửữựỳýỷỹỵđ]" .)
  □ All code blocks unchanged
  □ All YAML syntax valid (yamllint)
  □ All Markdown renders correctly
  □ Technical accuracy preserved
  □ Professional English (no machine translation artifacts)
```

### Step 5: Commit & Push (10 mins)
```yaml
Git Workflow:
  git add .agents/workflows/CODEX_EXPECTED_BEHAVIOR_SUMMARY.md
  git add .agents/lessons_learned/META_LEARNING_WHEN_TO_CREATE_LESSONS.md
  git add .agents/lessons_learned/REALITY_CHECK_PROVEN_VS_PROPOSED.md
  git add .agents/lessons_learned/CURSOR_PROTOCOL_VIOLATION_META_LESSON_2025-10-27.md
  git add .agents/workflows/SESSION_SUMMARY_2025-10-27.md
  
  git commit -m "docs(translate): translate 5 files from Vietnamese to English
  
  Translated files per LANGUAGE_POLICY.md:
  - CODEX_EXPECTED_BEHAVIOR_SUMMARY.md (80% Vietnamese → 100% English)
  - META_LEARNING_WHEN_TO_CREATE_LESSONS.md (60% → 100%)
  - REALITY_CHECK_PROVEN_VS_PROPOSED.md (40% → 100%)
  - CURSOR_PROTOCOL_VIOLATION_META_LESSON_2025-10-27.md (50% → 100%)
  - SESSION_SUMMARY_2025-10-27.md (10% → 100%)
  
  Quality checks:
  - ✅ Technical accuracy preserved
  - ✅ Professional English
  - ✅ Format/syntax intact
  - ✅ No Vietnamese characters remain
  
  Related: #56"
  
  git push
```

### Step 6: Announce Completion (5 mins)
```yaml
Action: Post in Issue #56
Message: "✅ Translation task complete. 5 files translated: Vietnamese → English.
  
  Files:
  - CODEX_EXPECTED_BEHAVIOR_SUMMARY.md ✅
  - META_LEARNING_WHEN_TO_CREATE_LESSONS.md ✅
  - REALITY_CHECK_PROVEN_VS_PROPOSED.md ✅
  - CURSOR_PROTOCOL_VIOLATION_META_LESSON_2025-10-27.md ✅
  - SESSION_SUMMARY_2025-10-27.md ✅
  
  Quality: Technical accuracy preserved, professional English, all syntax valid.
  Commit: [link to commit]"
```

---

## ✅ VALIDATION COMMANDS

### Check for Vietnamese Characters
```bash
# Should return NO matches
grep -r "[àáảãạâầấẩẫậăằắẳẵặèéẻẽẹêềếểễệìíỉĩịòóỏõọôồốổỗộơờớởỡợùúủũụưừứửữựỳýỷỹỵđ]" .agents/

# If matches found, those files still have Vietnamese
```

### Validate YAML Syntax
```bash
# For YAML files (if any)
yamllint --strict .agents/**/*.yml
```

### Validate Markdown
```bash
# Check Markdown renders correctly
markdownlint .agents/**/*.md || echo "Check warnings, fix if needed"
```

### Quick Preview
```bash
# Preview each file to verify translation quality
cat .agents/workflows/CODEX_EXPECTED_BEHAVIOR_SUMMARY.md | head -50
```

---

## 📊 EVALUATION CRITERIA

### Process Adherence (40 points)
```yaml
✅ Claimed task before starting: 10 points
✅ Followed workflow steps: 10 points
✅ Committed with proper message: 10 points
✅ Announced completion: 10 points
```

### Translation Quality (30 points)
```yaml
✅ Technical accuracy: 10 points
✅ Professional English: 10 points
✅ No machine translation artifacts: 10 points
```

### Format Preservation (20 points)
```yaml
✅ YAML syntax valid: 10 points
✅ Markdown renders correctly: 10 points
```

### Completeness (10 points)
```yaml
✅ All 5 files translated: 5 points
✅ No Vietnamese characters remain: 5 points
```

**Total**: 100 points  
**Pass Threshold**: 80 points  
**Excellence**: 90+ points

---

## 🚨 COMMON PITFALLS (Avoid These)

### Pitfall 1: Over-Translation
```yaml
❌ DON'T translate technical terms
Example:
  Wrong: "git branch" → "git nhánh" (if reverse translating)
  Right: "git branch" (keep in English)
```

### Pitfall 2: Breaking Syntax
```yaml
❌ DON'T modify YAML structure
Example:
  Wrong: Changing "status: pending" → "status: đang chờ"
  Right: Keep "status: pending" (technical field)
```

### Pitfall 3: Machine Translation Artifacts
```yaml
❌ DON'T use raw machine translation
Example:
  Wrong: "The file was created by me" (overly formal)
  Right: "Created the file" (professional, concise)
```

### Pitfall 4: Losing Context
```yaml
❌ DON'T translate sentences in isolation
Example:
  Vietnamese: "Tôi phát hiện lỗi này khi test"
  Wrong: "I discover this error when test" (bad grammar)
  Right: "Discovered this error during testing" (proper context)
```

---

## 📞 SUPPORT & ESCALATION

### If You Need Help
```yaml
Scenario 1: Ambiguous Technical Term
  Action: Post question in Issue #56
  Example: "Is 'golden master' a technical term or should translate?"
  Wait: For clarification before proceeding

Scenario 2: Syntax Uncertain
  Action: Test with validation commands first
  If still uncertain: Ask in Issue #56

Scenario 3: Translation Quality Concern
  Action: Provide 2-3 alternative translations
  Request: User feedback on preferred style
```

### Escalation Path
```yaml
Level 1: Self-check with validation commands
Level 2: Post question in Issue #56
Level 3: Request Cursor review (if needed)
```

---

## 🎯 SUCCESS INDICATORS

### You'll Know You're Successful When:
```yaml
✅ grep for Vietnamese returns 0 matches
✅ yamllint passes all files
✅ Markdown renders correctly in GitHub preview
✅ Technical terms unchanged (git, CI, PR, AA, etc.)
✅ Professional tone maintained
✅ User approves quality in Issue #56
✅ No follow-up corrections needed
```

---

## 📚 REFERENCE MATERIALS

### Read Before Starting
- `.agents/workflows/LANGUAGE_POLICY.md` (MUST READ)
- `.agents/lessons_learned/OPERATING_PRINCIPLES.md` (context)
- This spec completely

### Style References
- Technical writing: Clear, concise, professional
- Avoid: "very", "really", "quite" (filler words)
- Prefer: Active voice, present tense for current state

---

**Task Ready**: Gemini can claim this task and execute autonomously.  
**Estimated Completion**: 3 hours  
**Quality Target**: 90+ points (excellence)  
**Blocked By**: Nothing (ready to start)

---

**Created By**: Cursor (Claude 4.5 Sonnet)  
**Date**: 2025-10-27  
**Version**: 1.0  
**Status**: Ready for claim
