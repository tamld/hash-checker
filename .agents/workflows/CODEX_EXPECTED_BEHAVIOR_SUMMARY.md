# Codex Expected Behavior Summary

**Date**: 2025-10-27  
**Context**: Phase 2 delegation after Phase 1 completes  
**Purpose**: Quick reference for expected Codex behavior

---

## 🎯 **NHIỆM VỤ (TASK)**

```yaml
Task: Create COORDINATION_RULES.md
Constraints:
  - Exactly 3 rules (không nhiều hơn)
  - Max 50 lines total
  - Simple, memorable language
  - 20 minutes duration

Success: 80+ points on 100-point evaluation scale
```

---

## 📋 **5 BƯỚC MONG MUỐN**

### **Bước 1: Claim Task (2 phút)**

```yaml
Codex phải:
  ✅ Post công khai trong Issue #56: "I claim Phase 2"
  ✅ Nêu ETA: "ETA: 20 mins"
  ✅ Chờ confirmation từ Cursor (hoặc 5 mins)

Không được:
  ❌ Bắt đầu trước khi claim
  ❌ Claim riêng tư (phải công khai)
  ❌ Bỏ qua bước này
```

### **Bước 2: Đọc Context (5 phút)**

```yaml
Phải đọc 3 files:
  1. OPERATING_PRINCIPLES.md (focus: Principle 1, 7)
  2. COMPREHENSIVE_STRATEGY_REVIEW.md (focus: Phase 2 spec)
  3. agents_registry.md (focus: coordination section)

Hiểu rõ:
  - Tại sao cần simple (not complex)
  - Tại sao 3 rules (not 5 or 10)
  - Tại sao test first, formalize later
```

### **Bước 3: Thiết Kế 3 Rules (8 phút)**

```yaml
Pattern mong muốn:
  Rule 1: Announce (Thông báo trước khi push)
  Rule 2: Own (Sở hữu files của mình)
  Rule 3: Sync (Rebase trước khi push)

Anti-patterns tránh:
  ❌ Tạo 5-10 rules ("to be thorough")
  ❌ Thêm edge cases (premature optimization)
  ❌ Detailed conflict resolution (save for later)
  ❌ Technical deep-dives (keep high-level)
```

### **Bước 4: Implement & Self-Review (3 phút)**

```yaml
Tự kiểm tra:
  ☐ Exactly 3 rules? (count them)
  ☐ Under 50 lines? (wc -l)
  ☐ Simple language? (non-technical person can understand)
  ☐ Memorable? (can remember after 1 read)
  ☐ Would I follow these? (practical test)
```

### **Bước 5: Push & Announce (2 phút)**

```yaml
Git workflow ĐÚNG:
  1. Announce: "About to push COORDINATION_RULES.md"
  2. Wait 2 mins (cho conflicts)
  3. Sync: git pull --rebase
  4. Add: git add .agents/workflows/COORDINATION_RULES.md
  5. Commit: Clear message explaining 3 rules
  6. Push: git push
  7. Announce: "✅ Done. Ready for review"

Quan trọng:
  ✅ Announce BEFORE push (follow Rule 1!)
  ✅ Rebase BEFORE push (follow Rule 3!)
  ✅ Clear commit message
  ✅ Request review
```

---

## 📊 **ĐÁNH GIÁ (EVALUATION)**

### **100-Point Scale**

```yaml
40 points - Process Adherence:
  - Did claim publicly? (10 pts)
  - Read context? (10 pts)
  - Announced before push? (10 pts)
  - Synced before push? (10 pts)

30 points - Output Quality:
  - Exactly 3 rules? (10 pts)
  - Under 50 lines? (10 pts)
  - Simple language? (10 pts)

20 points - Collaboration:
  - Clear communication? (10 pts)
  - Requested review? (10 pts)

10 points - Wisdom Applied:
  - Avoided over-engineering? (5 pts)
  - Showed restraint? (5 pts)

Pass: 80+ points
Excellence: 90+ points
```

---

## ⚠️ **4 LỖI PHỔ BIẾN (PITFALLS)**

### **Lỗi 1: Over-Engineering**

```yaml
Biểu hiện: Tạo 5-10 rules "để hoàn chỉnh"
Tại sao sai: Vi phạm nguyên tắc simplicity
Cursor sẽ: Yêu cầu simplify lại
```

### **Lỗi 2: Bỏ Qua Protocols**

```yaml
Biểu hiện: Bắt đầu làm trước khi claim
Tại sao sai: Vi phạm coordination đang xây dựng
Cursor sẽ: Document as protocol violation
```

### **Lỗi 3: Không Follow Own Rules**

```yaml
Biểu hiện: Push không announce trước
Tại sao sai: Ironic - tạo rules nhưng không follow
Cursor sẽ: Highlight as meta-lesson
```

### **Lỗi 4: Premature Optimization**

```yaml
Biểu hiện: Thêm "edge case handling" sections
Tại sao sai: Chưa test basic rules
Cursor sẽ: Request removal
```

---

## ✅ **MẪU OUTPUT MONG MUỐN**

### **File Structure**

```markdown
# Multi-Agent Coordination Rules (Simple)

## 3 Rules (THAT'S IT!)

### Rule 1: Announce Before Push
Post in Issue/PR: "Working on [files], ETA [time]"
Wait 5 mins for conflicts

### Rule 2: Own Your Files
Create: {your_name}_*.md for your work
Don't edit others' {name}_*.md files

### Rule 3: Sync Before Push
git fetch && git pull --rebase
Test still works
Then push

## Conflict Resolution
If conflict: Create CONFLICT_{topic}.md
Document both sides, tag human, wait for decision

## That's It!
3 rules. Keep it simple.
```

**Total**: ~30 lines  
**Quality**: Simple, clear, memorable ✅

---

## 🎓 **ĐIỀU BẠN MUỐN QUAN SÁT**

### **Technical Capabilities**

```yaml
Can Codex:
  ✅ Follow multi-step spec accurately?
  ✅ Understand "simple" vs "comprehensive"?
  ✅ Exercise restraint (not add extras)?
  ✅ Follow git workflow correctly?
```

### **Behavioral Patterns**

```yaml
Does Codex:
  ⚠️ Over-engineer by default? (create 10 rules)
  ⚠️ Skip protocols? (start without claiming)
  ⚠️ Argue with spec? (defend 5 rules vs 3)
  ✅ Accept feedback? (iterate gracefully)
```

### **Wisdom Application**

```yaml
Does Codex show:
  ✅ Understanding of operating principles?
  ✅ Ability to apply "simplicity is earned"?
  ✅ Recognition of "reality > hypothesis"?
  ✅ Meta-awareness (following own rules)?
```

---

## 📈 **EXPECTED OUTCOMES**

### **Ideal Scenario (90+ points)**

```yaml
Codex:
  ✅ Claims task professionally
  ✅ Reads all context files
  ✅ Creates exactly 3 rules (~40 lines)
  ✅ Announces before pushing
  ✅ Syncs before pushing
  ✅ Requests review
  ✅ Shows restraint (doesn't over-engineer)

Result:
  ✅ Perfect delegation model
  ✅ Framework validated
  ✅ Ready for Phase 3
```

### **Good Scenario (80-89 points)**

```yaml
Codex:
  ✅ Follows most protocols
  ✅ Creates simple rules (maybe 4 instead of 3)
  ✅ Minor iteration needed
  ✅ Accepts feedback well

Result:
  ✅ Framework works with minor tweaks
  ⚠️ Spec needs clarification
  ✅ Can proceed to Phase 3
```

### **Problematic Scenario (<80 points)**

```yaml
Codex:
  ❌ Skips claim process
  ❌ Creates 10 rules
  ❌ Doesn't announce/sync
  ❌ Argues with spec

Result:
  ⚠️ Framework needs major revision
  ⚠️ Codex needs more training
  ⏸️ Pause multi-AA rollout
```

---

## 🔄 **SAU KHI HOÀN THÀNH**

### **Cursor Sẽ Làm Gì**

```yaml
1. Review output (5 mins):
   - Check requirements met
   - Evaluate quality
   - Note deviations

2. Fill evaluation form (10 mins):
   - Use AA_PERFORMANCE_EVALUATION_TEMPLATE.md
   - Score 4 dimensions
   - Document lessons learned

3. Provide feedback (5 mins):
   - What went well
   - What could improve
   - Iterate if needed

4. Document meta-lessons (10 mins):
   - Framework validation
   - Spec quality assessment
   - Improvements for future
```

### **Nếu Cần Iteration**

```yaml
Cursor requests changes:
  "Please simplify Rule 2 to 1 sentence"

Codex expected response:
  "Thanks for feedback. Revised Rule 2. Pushing v2."

NOT:
  "But I think 2 sentences is better because..."
  (Arguing vs following)
```

---

## 🎯 **SUCCESS METRICS**

### **Process Success**

```yaml
✅ Codex claimed before starting
✅ Codex read context files
✅ Codex announced before pushing
✅ Codex synced before pushing
✅ Codex requested review

Score: Pass if ALL checked
```

### **Output Success**

```yaml
✅ Exactly 3 rules
✅ Under 50 lines
✅ Simple, clear language
✅ No over-engineering

Score: Pass if ALL checked
```

### **Meta Success (Quan Trọng Nhất!)**

```yaml
✅ Framework works in practice (not just theory)
✅ Operating principles can be transmitted
✅ Multi-AA coordination is viable
✅ Evaluation methodology is effective
✅ Can scale to more AAs

Score: Pass if 4/5 checked
```

---

## 📞 **KHI NÀO INTERVENE**

### **Cursor Nên Can Thiệp**

```yaml
Intervene immediately if:
  🚨 Codex starts without claiming (protocol violation)
  🚨 Codex pushes without announcing (safety issue)
  🚨 Codex creates 10+ rules (requirement violation)

Let Codex self-correct if:
  ⏸️ Creates 4 rules (close, can iterate)
  ⏸️ 55 lines instead of 50 (minor, can trim)
  ⏸️ Different approach but meets criteria (flexibility)
```

### **Khi Nào Request Iteration**

```yaml
Request iteration if:
  ⚠️ 5+ rules (too complex)
  ⚠️ 80+ lines (too detailed)
  ⚠️ Technical jargon (not simple)
  ⚠️ Edge cases documented (premature)

Accept as-is if:
  ✅ 3 rules, simple, clear
  ✅ Minor style differences (not critical)
  ✅ Different wording but same intent
```

---

## 📚 **DOCUMENTS REFERENCE**

### **For Codex to Read**

```yaml
Before starting:
  1. CODEX_DELEGATION_SPEC_PHASE2.md (full spec)
  2. OPERATING_PRINCIPLES.md (wisdom)
  3. COMPREHENSIVE_STRATEGY_REVIEW.md (context)
```

### **For Cursor to Use**

```yaml
During evaluation:
  1. AA_PERFORMANCE_EVALUATION_TEMPLATE.md (scoring)
  2. CODEX_DELEGATION_SPEC_PHASE2.md (reference)
  3. This file (quick checklist)
```

---

## ✅ **QUICK CHECKLIST (For You)**

### **Sau Khi Phase 1 Xong**

```yaml
☐ Announce in Issue #56: "Phase 2 available for Codex"
☐ Tag Codex (if possible)
☐ Wait for Codex to claim
☐ Confirm claim: "Approved, see CODEX_DELEGATION_SPEC"
☐ Monitor progress (but don't micromanage)
☐ Wait for Codex to finish
☐ Review output
☐ Fill evaluation form
☐ Provide feedback
☐ Document meta-lessons
☐ Proceed to Phase 3 (if successful)
```

---

## 🎯 **TÓM TẮT 1 DÒNG**

> **Mong muốn**: Codex claim task → đọc context → tạo 3 rules đơn giản → announce → sync → push → request review → iterate if needed → 80+ points → proceed to Phase 3.

---

**Author**: Cursor  
**For**: User (quick reference)  
**Status**: Ready to use after Phase 1  
**Expected Duration**: Entire delegation + evaluation ~1 hour
