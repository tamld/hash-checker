# Language Policy for Multi-AA Project

**Date**: 2025-10-27  
**Status**: MANDATORY (all AAs must follow)  
**Purpose**: Clear, consistent communication across multi-language team

---

## 🌍 **POLICY STATEMENT**

### **Core Principle**

```yaml
User Communication: Vietnamese (Tiếng Việt)
  - Chat/conversation with @tamld
  - Questions and answers
  - Verbal discussions
  - Real-time interaction

All Documentation: English
  - Code comments
  - Markdown files (.md)
  - YAML configs
  - Commit messages
  - Issues/PRs
  - Technical documentation
  - Lessons learned
  - Brainstorms
  - ALL files in repository

Exception: NONE
  - No mixed language files
  - No Vietnamese in documentation
  - No English in user conversations (unless technical terms)
```

---

## 📋 **RATIONALE**

### **Why English for Documentation?**

```yaml
Reason 1: International Collaboration
  - Code/docs shared globally
  - Other developers may contribute
  - English is universal tech language

Reason 2: AI Training & Tools
  - Better AI understanding (English-trained models)
  - IDE tools expect English
  - Stack Overflow / docs are English

Reason 3: Professional Standards
  - Industry standard: Code in English
  - Open source best practice
  - Future-proof (team may expand)

Reason 4: Consistency
  - Single language = clearer
  - No context switching
  - Easier to search/grep
```

### **Why Vietnamese for User Communication?**

```yaml
Reason 1: User Preference
  - @tamld prefers Vietnamese
  - Native language = clearer thinking
  - Nuance preserved

Reason 2: Efficiency
  - Faster communication
  - Less translation overhead
  - Natural expression

Reason 3: Context
  - Complex concepts easier in native language
  - Cultural context preserved
```

---

## ✅ **IMPLEMENTATION GUIDELINES**

### **For All AAs (Cursor, Codex, Gemini)**

#### **1. User-Facing Communication**

```yaml
Format: Vietnamese

Examples:
  ✅ Good:
    "Được rồi, để tôi tạo file đó."
    "Có vấn đề gì với approach này không?"
    "Tôi đã hoàn thành Phase 1."
  
  ❌ Bad:
    "OK, I'll create that file."
    "Any issues with this approach?"
    "I completed Phase 1."

Technical Terms:
  - Keep in English when no good Vietnamese equivalent
  - Example: "Tôi sẽ tạo PR" (not "Tôi sẽ tạo Yêu Cầu Kéo")
  - Example: "CI đã pass" (not "Tích hợp liên tục đã vượt qua")
```

#### **2. Documentation & Code**

```yaml
Format: English ONLY

File Types:
  ✅ .md files: English
  ✅ .rs files: English (code + comments)
  ✅ .yml files: English
  ✅ .toml files: English
  ✅ .sh files: English
  ✅ ALL repository files: English

Commit Messages:
  ✅ English only
  Example: "docs: add language policy"
  ❌ Never: "docs: thêm chính sách ngôn ngữ"

Issue/PR Titles:
  ✅ English only
  Example: "fix: protocol violation in coordination workflow"
  ❌ Never: "fix: vi phạm protocol trong quy trình phối hợp"

Comments:
  ✅ English
  Example: "// Check if task is already claimed"
  ❌ Never: "// Kiểm tra task đã được claim chưa"
```

#### **3. Mixed Content Handling**

```yaml
Scenario: User asks in Vietnamese, AA responds with code

Correct Approach:
  1. Respond in Vietnamese (acknowledgment)
  2. Create English documentation
  3. Explain in Vietnamese what was created

Example:
  User: "Tạo file để document lesson này"
  
  AA Response (Vietnamese):
    "Được rồi, tôi sẽ tạo lesson document bằng tiếng Anh 
     trong .agents/lessons_learned/"
  
  File Created (English):
    # Lesson: Protocol Adherence in Multi-AA Environment
    ...all content in English...
  
  Follow-up (Vietnamese):
    "Đã tạo file LESSON_PROTOCOL_ADHERENCE.md. 
     Bạn muốn tôi điều chỉnh gì không?"
```

---

## 🔍 **QUALITY CHECKS**

### **Pre-Commit Checklist**

```yaml
Before committing ANY file:
  
  ☐ Is this a documentation file? (.md, .txt, etc.)
    → IF yes: Verify 100% English
  
  ☐ Are there Vietnamese characters?
    → IF yes in documentation: ❌ STOP, translate to English
    → IF yes in user conversation: ✅ OK
  
  ☐ Are commit messages in English?
    → IF no: ❌ STOP, rewrite in English
  
  ☐ Are code comments in English?
    → IF no: ❌ STOP, translate to English
```

### **Automated Checks (Future)**

```yaml
Git Hook (pre-commit):
  - Scan .md files for Vietnamese characters
  - Flag if found
  - Prevent commit until fixed

CI Check:
  - Lint documentation files
  - Check commit message language
  - Fail build if Vietnamese in docs
```

---

## 🛠️ **VIOLATION REMEDIATION**

### **If Vietnamese Found in Documentation**

```yaml
Step 1: Identify all files with Vietnamese
  Command: 
    grep -r "à\|á\|ả\|ã\|ạ\|ă\|ắ\|ằ\|ẳ\|ẵ\|ặ\|â\|ấ\|ầ\|ẩ\|ẫ\|ậ" .agents/

Step 2: Translate to English
  - Use clear, technical English
  - Preserve meaning, not literal translation
  - Use standard terminology

Step 3: Review translation
  - Check for clarity
  - Ensure technical accuracy
  - Verify no Vietnamese remains

Step 4: Commit fix
  Message: "docs: translate [file] from Vietnamese to English"

Step 5: Update language policy if needed
  - Was policy unclear?
  - Add clarification
```

### **Current Violations (Need Fixing)**

```yaml
Files Created Today with Vietnamese Content:
  ❌ META_LEARNING_WHEN_TO_CREATE_LESSONS.md
  ❌ REALITY_CHECK_PROVEN_VS_PROPOSED.md
  ❌ CURSOR_PROTOCOL_VIOLATION_META_LESSON_2025-10-27.md
  ❌ AI_VISION_GUI_VERIFICATION_ANALYSIS.md
  ❌ GUI_TESTING_WORKFLOW_MULTI_AA_BRAINSTORM.md
  ❌ CODEX_EXPECTED_BEHAVIOR_SUMMARY.md (intentionally Vietnamese for quick ref)

Action Required:
  1. Review each file
  2. Translate Vietnamese sections to English
  3. Commit translations
  4. Update this policy with lessons learned
```

---

## 📚 **EXAMPLES & TEMPLATES**

### **Document Header Template**

```markdown
# [Document Title in English]

**Date**: YYYY-MM-DD  
**Purpose**: [Clear English description]  
**Status**: [Draft | Review | Final]

---

## Section 1

[All content in English]

## Section 2

[All content in English]
```

### **Commit Message Template**

```bash
# Format: type(scope): subject
# Example:
docs(agents): add language policy and translation guidelines

# ✅ Good examples:
feat(gui): add screenshot comparison workflow
fix(ci): resolve merge conflict in workflow file
docs(lessons): translate meta-learning document to English

# ❌ Bad examples:
docs: thêm chính sách ngôn ngữ
fix: sửa conflict trong file workflow
```

### **Code Comment Template**

```rust
// ✅ Good (English):
// Check if task is already claimed before proceeding
// Returns true if task is available, false otherwise
fn is_task_available(task_id: &str) -> bool {
    // Implementation
}

// ❌ Bad (Vietnamese):
// Kiểm tra task đã được claim chưa
// Trả về true nếu task còn trống
fn is_task_available(task_id: &str) -> bool {
    // Implementation
}
```

---

## 🎯 **ENFORCEMENT**

### **Responsibility**

```yaml
Every AA is responsible for:
  ✅ Following language policy
  ✅ Checking own output before commit
  ✅ Translating if Vietnamese found
  ✅ Maintaining English documentation

No Exceptions:
  - Cursor must follow (no designer privilege)
  - Codex must follow
  - Gemini must follow
  - All future AAs must follow
```

### **Review Process**

```yaml
PR Review Checklist:
  ☐ All .md files in English?
  ☐ All code comments in English?
  ☐ Commit messages in English?
  ☐ No Vietnamese in documentation?
  
  IF any ☐ unchecked:
    → Request changes
    → Provide translation
    → Re-review after fix
```

---

## ✅ **ACCEPTANCE CRITERIA**

### **This Policy is Effective When:**

```yaml
Metrics:
  - 100% of documentation files in English
  - 100% of commit messages in English
  - 100% of code comments in English
  - 0 Vietnamese characters in .md files (except examples)
  - User communication remains Vietnamese

Current Status:
  - Documentation: ~60% English (many violations!)
  - Commit messages: 95% English
  - Code comments: 100% English
  - User communication: 100% Vietnamese ✅

Goal:
  - Documentation: 100% English by end of today
  - All metrics at 100% going forward
```

---

## 🔄 **NEXT STEPS**

### **Immediate Actions**

```yaml
1. Translate existing Vietnamese documents to English
   Priority: HIGH
   Owner: Cursor
   Deadline: Today

2. Commit this language policy
   Priority: CRITICAL
   Owner: Cursor
   Deadline: Now

3. Reference policy in AGENTS.md
   Priority: HIGH
   Owner: Cursor
   Deadline: Today

4. Apply policy to all future work
   Priority: MANDATORY
   Owner: All AAs
   Deadline: Ongoing
```

---

## 📖 **REFERENCE**

### **Related Documents**

```yaml
- AGENTS.md: References this language policy
- CONTRIBUTING.md: Should reference this policy
- README.md: Project description (English)
- All documentation: Must follow this policy
```

### **Questions?**

```yaml
If unclear about language choice:
  1. Is it user-facing conversation? → Vietnamese
  2. Is it documentation/code? → English
  3. Still unclear? → Ask user for clarification
```

---

**Policy Status**: ACTIVE  
**Mandatory Compliance**: YES  
**Exceptions**: NONE

---

**Last Updated**: 2025-10-27  
**Version**: 1.0  
**Owner**: All AAs in project
