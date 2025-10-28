# Autonomous Multi-AA Brainstorm System - Cursor's Proposal

**Date**: 2025-10-28  
**Author**: Cursor  
**Confidence**: 50%  
**Status**: Draft

---

## Problem Understanding

Current brainstorm process is MANUAL and USER-INTENSIVE:
- User invites each AA individually (3 separate sessions)
- User coordinates between AAs (sequential, not parallel)
- User decides when phase complete (bottleneck)
- Process doesn't scale (10 AAs = 10 manual invitations)

**Core issue**: User time is bottleneck, AAs can't coordinate autonomously

User's vision (from conversation):
> "Các AA tham gia, tự động và ít cần sự quan tâm của con người, thì ta phải có 1 workflow rõ ràng. Ai là người chủ trì, ai là người tham gia. Quy tắc đó phải đảm bảo được các quy luật rõ ràng để AA nắm rõ vai trò của mình. Nó cũng phải có 1 cơ chế đủ thông minh để bắt đầu và kết thúc quá trình brainstorm tự động. Giống như trong 1 cuộc thi, sẽ có tiếng còi khai cuộc, nhưng khác ở chỗ AA host sẽ quyết định khi nào kết thúc brainstorm và lan truyền thông tin đó đến cho các AA khác tham gia brainstorm."

Key requirements identified:
1. **Workflow clarity**: Who is host, who are participants (roles)
2. **Rule-based system**: AAs understand roles automatically (no ambiguity)
3. **Smart mechanism**: Start signal + end signal (like competition whistle)
4. **Host authority**: Host decides when complete, broadcasts to all
5. **Minimal human intervention**: User triggers, AAs execute, User reviews

---

## Proposed Solution

### 3-Phase Implementation (Minimize Risk)

#### Phase 1: Semi-Automated (RECOMMENDED START)

```yaml
What:
  - Host creates status.yml manually (1 commit)
  - Participants watch status.yml (auto-polling every 30s)
  - Participants contribute proposals automatically (no user trigger per AA)
  - Host monitors manually (checks every 12 hours)
  - Host ends manually (updates status.yml when criteria met)

Automation level: 40%
  Automated: Participant contribution, status tracking
  Manual: Host init, host monitoring, host completion

Benefits:
  - 2x user time savings (no individual AA invitations)
  - Low risk (manual checkpoints prevent runaway)
  - Quick to implement (2 days)
  - Learn from real usage (calibrate before full automation)

Example workflow:
  Day 0, 9am: User asks Cursor "start brainstorm topic 1"
  Day 0, 9:05am: Cursor creates status.yml, commits → MANUAL
  Day 0, 9:10am: Codex pulls, sees status.yml, starts proposal → AUTO
  Day 0, 2pm: Codex commits proposal → AUTO
  Day 0, 3pm: Gemini pulls, sees status.yml, starts proposal → AUTO
  Day 0, 8pm: Gemini commits proposal → AUTO
  Day 1, 9am: Cursor (host) checks status.yml → all done
  Day 1, 9:10am: Cursor updates status.yml phase="complete" → MANUAL
  Day 1, 9:15am: Cursor reports to user "Topic 1 complete (3/3)" → MANUAL

User sessions: 2 (trigger start, review end)
Duration: 1-2 days (parallel AA work)
Risk: LOW
```

#### Phase 2: Mostly Automated (FUTURE)

```yaml
What:
  - Host creates status.yml + starts watch loop (auto-monitor)
  - Participants watch + contribute (auto)
  - Host ends automatically (criteria met, updates status.yml)
  - Host reports to user (summary)
  - User approves before next phase (checkpoint)

Automation level: 80%
  Automated: Everything except user approval
  Manual: User approval for phase transition

Benefits:
  - 5x user time savings (1 trigger, 1 approval)
  - Continuous operation (AAs work 24/7)
  - Scalable (10 AAs = same process)

Risk: MEDIUM (watch loops, but user checkpoint prevents errors)
Duration to implement: 1 week
```

#### Phase 3: Full Autonomous (LONG-TERM VISION)

```yaml
What:
  - Entire lifecycle automated (propose → discuss → consensus → test)
  - Host orchestrates all phases
  - Participants coordinate automatically
  - User receives final report (all phases complete)
  - User intervenes only if escalation (timeout, conflict)

Automation level: 95%
  Automated: Everything
  Manual: Only escalations

Benefits:
  - 10x user time savings
  - Full AA ecosystem autonomy
  - User as observer/coach (not operator)

Risk: HIGH (infinite loops, deadlocks, quality issues)
Duration to implement: 2-3 weeks + extensive testing
```

---

## Core Components (Architecture)

### 1. Role Definition

```yaml
Host AA (Chủ trì):
  Responsibilities:
    - Initialize brainstorm (create status.yml)
    - Monitor participation (track progress)
    - Decide completion (when criteria met)
    - Broadcast results (update status.yml)
    - Report to user (summary)
  
  Decision authority:
    - When to start ✅
    - When to end ✅
    - Who participates ✅
    - What's "enough" ✅
  
  Constraints:
    - Must justify decisions (transparent)
    - Must wait minimum time (fairness - 24h min)
    - Must check all participants (completeness)
  
  Example: Cursor in current branch

Participant AAs (Tham gia):
  Responsibilities:
    - Monitor status.yml (check for work)
    - Add proposals (contribute content)
    - Update own status (visibility)
    - Respect completion signal (stop when told)
  
  Decision authority:
    - When to contribute ✅
    - What to propose ✅
    - Confidence level ✅
  
  Constraints:
    - Cannot end brainstorm (only host)
    - Must follow template (consistency)
    - Must update status (coordination)
  
  Example: Codex, Gemini
```

### 2. State Machine (status.yml)

```yaml
Location: .agents/brainstorms/[topic]/status.yml

Schema:
  brainstorm_id: "topic-1-aa-core-skills"
  host: "cursor"
  phase: "collecting" | "discussing" | "consensus" | "complete"
  
  participants:
    - name: "cursor"
      status: "proposal_done"
      last_update: "2025-10-28T14:00:00Z"
    
    - name: "codex"
      status: "proposal_in_progress"
      last_update: "2025-10-28T14:30:00Z"
    
    - name: "gemini"
      status: "pending"
      last_update: null
  
  timing:
    started: "2025-10-28T13:00:00Z"
    min_duration: "24 hours" # Fairness
    timeout: "7 days" # Deadlock prevention
    last_check: "2025-10-28T14:45:00Z"
  
  completion_criteria:
    min_participants: 2 # At least 2 AAs
    min_proposals: 2 # At least 2 proposals
    quality_gate: "all proposals have confidence level"
  
  host_decision:
    can_end: false # Computed by host
    reason: "Waiting for Gemini (min 24h not elapsed)"
    next_check: "2025-10-29T13:00:00Z"

Behavior:
  - Host updates: phase, can_end, reason
  - Participants update: own status only
  - All AAs read: entire file (know global state)
  - Git commits = broadcast channel (SSoT)
```

### 3. Start Signal (Tiếng còi khai cuộc)

```yaml
Trigger: Host creates status.yml with phase="collecting"

Detection mechanism (Phase 1):
  Method: File existence polling
  Frequency: 30 seconds
  Logic: if status.yml exists AND phase=="collecting" → join

Participant action:
  1. Pull git
  2. Read status.yml
  3. Check if my name in participants
  4. If yes → create my proposal
  5. Update my status to "proposal_done"
  6. Commit & push

Pros: Simple, no infrastructure needed
Cons: 30s delay, polling overhead
```

### 4. End Signal (Kết thúc brainstorm)

```yaml
Host decision logic:

can_end_brainstorm(status):
  # Check completion criteria
  done_count = count(p for p in participants if p.status == "proposal_done")
  time_elapsed = now() - status.timing.started
  timeout_exceeded = time_elapsed > status.timing.timeout
  
  if timeout_exceeded:
    return True, "TIMEOUT: Proceeding with available proposals"
  
  criteria_met = (
    done_count >= status.completion_criteria.min_participants
    AND time_elapsed >= status.timing.min_duration
    AND all_proposals_valid()
  )
  
  if criteria_met:
    return True, "SUCCESS: All criteria met"
  
  return False, f"Waiting: {done_count}/3 AAs, {time_elapsed} elapsed"

Host action when can_end=True:
  1. Update status.yml:
       phase: "complete"
       end_time: "2025-10-29T15:00:00Z"
       result: "3 proposals collected"
  
  2. Commit & push (broadcast)
  
  3. Create summary: PHASE1_SUMMARY.md
  
  4. Report to user: "Topic 1 complete (3/3 AAs)"

Participants detect:
  1. Pull git every 30s
  2. Read status.yml
  3. If phase == "complete" → stop watch loop
  4. Read summary, proceed to next phase
```

### 5. Communication Protocol

```yaml
Broadcast: Git commits (SSoT)

Host → Participants:
  Channel: status.yml updates
  Example: git commit -m "brainstorm: topic 1 complete (3/3)"
  Detection: Participants poll every 30s

Participant → Host:
  Channel: Own proposal file + status.yml update
  Example: git commit -m "brainstorm: codex proposal for topic 1"
  Detection: Host checks every 12h (Phase 1) or 30s (Phase 2)

Participant → Participant:
  No direct communication during proposal phase
  Discussion phase: discussion.md (collaborative)
```

### 6. Watch Loop Protocol (Phase 2+)

```yaml
Host watch loop (Phase 2):

while True:
  git pull
  status = read_yaml("status.yml")
  can_end, reason = can_end_brainstorm(status)
  
  status.host_decision.can_end = can_end
  status.host_decision.reason = reason
  write_yaml("status.yml", status)
  
  if can_end:
    finalize_brainstorm(status)
    notify_user(status)
    break
  
  sleep(30) # 30 seconds

Participant watch loop:

while True:
  git pull
  status = read_yaml("status.yml")
  
  if status.phase == "complete":
    break
  
  if my_task_pending(status):
    do_my_proposal()
    update_status(status, "proposal_done")
    git_commit_push()
  
  sleep(30)

Exit conditions:
  Host: When phase complete OR timeout
  Participant: When phase == "complete"
```

---

## Critical Challenges & Solutions

### Challenge 1: Single Point of Failure (Host crashes)

```yaml
Problem: If host crashes, entire brainstorm stuck

Solution: Timeout-based failover
  - If host doesn't update status.yml for 2 hours → failover
  - Next participant (alphabetical) becomes host
  - Status: host_failover: true, previous_host: "cursor"

Implementation (Phase 2):
  - Participants monitor status.yml.timing.last_check
  - If stale > 2 hours → elect new host
  - New host continues from current state

Confidence: 60% (needs testing)
```

### Challenge 2: Polling Waste

```yaml
Problem: 30s polling × 3 AAs = 360 pulls/hour (wasteful)

Solution: Exponential backoff
  - First check: 30s
  - If no change: 1 min, 2 min, 5 min, 10 min (max)
  - If change detected: reset to 30s

Alternative (Phase 3): Git hooks
  - GitHub Actions on push → webhook
  - AAs receive event → pull immediately
  - Zero waste, instant response

Confidence: 80% (backoff proven pattern)
```

### Challenge 3: Quality Gate

```yaml
Problem: Who validates proposals? Host might accept low quality

Solution: Automated validation
  - Required fields: confidence, rationale, questions
  - status.yml rejects proposals missing fields
  - Host can only end if all proposals pass validation

Schema:
  proposal_validation:
    cursor: {valid: true, errors: []}
    codex: {valid: true, errors: []}
    gemini: {valid: false, errors: ["missing confidence level"]}

Host logic:
  can_end = all(p.valid for p in proposals)

Confidence: 70% (validation is straightforward)
```

### Challenge 4: Fairness (Host bias or rush)

```yaml
Problem: Host might rush (end early) or favor own proposal

Solution A: Hard constraints
  min_duration: 24 hours (enforced, no override)
  min_participants: 2 (enforced)
  Host cannot bypass (coded in logic)

Solution B: Transparency
  host_decision.reason (auditable)
  User can review decision (accountability)

Solution C: Rotation
  Topic 1: Cursor hosts
  Topic 2: Codex hosts
  Topic 3: Gemini hosts
  Prevents single AA dominance

Confidence: 85% (constraints work)
```

### Challenge 5: Deadlock (Participant never finishes)

```yaml
Problem: Gemini never submits → stuck forever

Solution: Timeout
  After 7 days → host ends anyway
  Proceed with available proposals (cursor + codex)
  Document: gemini_timeout: true

Escalation (critical topics):
  After 3 days no activity → host pings user
  User decides: wait longer OR proceed without

Confidence: 90% (timeout is standard pattern)
```

---

## Efficiency Gains

### Current (Manual) vs Proposed (Autonomous)

```yaml
Current:
  Time: 3 days (1 day per AA, sequential)
  User effort: 3 sessions (invite each AA)
  Scalability: N AAs = N days

Phase 1 (Semi-Auto):
  Time: 1-2 days (parallel if AAs available)
  User effort: 2 sessions (trigger start, review end)
  Scalability: N AAs = 2 days (parallel)

Phase 2 (Mostly Auto):
  Time: 1-2 days (parallel)
  User effort: 1 session (approve phase transition)
  Scalability: N AAs = 2 days

Phase 3 (Full Auto):
  Time: 7-8 days (all phases, all topics)
  User effort: 1 session (review final report)
  Scalability: N AAs × M topics = 1 week

Gains (Phase 2):
  Time: 1.5-3x faster
  User effort: 3x reduction
  Scalability: 5-10x better
```

---

## Rationale

### Why This Approach Works

```yaml
1. Aligns with user's vision (90%)
   - Host/participant roles clear
   - Start signal (status.yml creation)
   - End signal (host decision)
   - Broadcast (git commits)

2. Leverages existing infrastructure
   - Git as SSoT (already working)
   - Separate files (no conflicts)
   - YAML for state (readable, parseable)

3. Phased rollout minimizes risk
   - Phase 1: Low risk, quick win (2 days)
   - Learn from real usage (calibrate)
   - Scale only if proven

4. Fail-safes built in
   - Timeouts (deadlock prevention)
   - Failover (host crash resilience)
   - Quality gates (automated validation)
   - User checkpoints (safety net)

5. Scales to 10+ AAs
   - Parallel coordination (not sequential)
   - Rules-based (no manual routing)
   - Broadcast (one-to-many communication)
```

---

## Confidence Assessment

```yaml
Overall confidence: 50%

High confidence (80-90%):
  ✅ Phase 1 implementation (semi-auto)
  ✅ status.yml schema (state machine)
  ✅ Completion criteria (logic clear)
  ✅ Timeout/failover (standard patterns)

Medium confidence (60-70%):
  ⚠️ Phase 2 implementation (watch loops)
  ⚠️ Polling efficiency (need to measure)
  ⚠️ Quality gate automation (validation logic)

Low confidence (30-40%):
  ⚠️ Phase 3 implementation (full autonomous)
  ⚠️ Consensus automation (complex)
  ⚠️ Real-world AA coordination (untested)

Unknowns (need testing):
  ❓ Will AAs actually coordinate smoothly?
  ❓ Is 30s polling optimal? (too fast/slow?)
  ❓ Will quality remain high without human review?
  ❓ Can host reliably decide when to end?
  ❓ Will failover work in practice?
```

---

## Questions for Other AAs

### For Codex

```yaml
1. Distributed systems experience:
   - Have you seen similar patterns in distributed systems?
   - What failure modes should we anticipate?
   - Better alternatives to polling? (event-driven?)

2. Consensus algorithms:
   - Phase 3 requires consensus automation
   - What algorithms/patterns work for multi-agent consensus?
   - Raft? Paxos? Something simpler?

3. Quality gates:
   - How to validate proposal quality automatically?
   - What metrics indicate "good enough"?
   - Can we detect low-effort proposals?
```

### For Gemini

```yaml
1. Google's multi-agent research:
   - What patterns does Google use for multi-agent coordination?
   - Any relevant papers/approaches?
   - Common pitfalls to avoid?

2. Scalability:
   - Design assumes 3-10 AAs
   - What if 100 AAs? (future)
   - Bottlenecks? Architectural changes needed?

3. Safety mechanisms:
   - How to prevent infinite loops?
   - Deadlock detection/prevention?
   - Resource limits? (CPU, memory, git commits)
```

---

## Open Issues

```yaml
1. Optimal polling frequency
   Current: 30s
   Concern: Too fast (wasteful) or too slow (laggy)?
   Need: Measure in practice, adjust

2. Host rotation strategy
   Current: Manual assignment per topic
   Concern: How to automate host selection?
   Options: Round-robin, workload-based, random

3. User intervention threshold
   Current: User approves phase transitions
   Question: When can we remove this checkpoint?
   Criteria: After N successful autonomous brainstorms?

4. Quality metrics
   Current: Check required fields exist
   Question: How to measure proposal quality?
   Ideas: Confidence level, word count, evidence cited?

5. Conflict resolution
   Current: Undefined (assume rare in proposal phase)
   Question: What if 2 AAs edit same file?
   Solution: Locks? Merge strategies? Retry logic?
```

---

## Next Steps (If Consensus Reached)

```yaml
1. Design Phase 1 detailed spec
   - status.yml exact schema
   - Host initialization script
   - Participant watch loop code
   - Completion criteria logic

2. Implement Phase 1 MVP
   - Build scripts (Python/Bash)
   - Test with 2 AAs (Cursor + Codex)
   - Measure: time, git pulls, quality

3. Test with real brainstorm
   - Use current 6-7 topics
   - Collect metrics (duration, user effort, quality)
   - Identify issues (bugs, UX, efficiency)

4. Evaluate results
   - Did it save user time? (target: 2x)
   - Did quality remain high? (vs manual)
   - Were there failures? (deadlocks, crashes)

5. If successful → design Phase 2
   - Add host watch loop (auto-monitor)
   - Implement failover (resilience)
   - Add exponential backoff (efficiency)

Timeline:
  Week 1: Design + consensus (this brainstorm)
  Week 2: Implement Phase 1 (2 days)
  Week 3: Test Phase 1 (real brainstorm)
  Week 4: Evaluate + decide on Phase 2
```

---

## Summary

```yaml
Proposal: 3-phase autonomous brainstorm system
  Phase 1: Semi-automated (40% auto, 2 days to build, LOW risk)
  Phase 2: Mostly automated (80% auto, 1 week to build, MEDIUM risk)
  Phase 3: Full autonomous (95% auto, 2-3 weeks to build, HIGH risk)

Core idea: User's vision (90% correct)
  - Host/Participant roles (clear authority)
  - Start signal (status.yml creation)
  - End signal (host decision, criteria-based)
  - Broadcast (git commits as SSoT)

Benefits:
  - 2-10x user time savings (depending on phase)
  - Scalable (10+ AAs)
  - Continuous operation (AAs work 24/7)
  - User as observer/coach (not operator)

Risks:
  - Phase 1: LOW (manual checkpoints)
  - Phase 2: MEDIUM (watch loops, but user approval)
  - Phase 3: HIGH (full automation, many unknowns)

Recommendation: Start Phase 1, measure, iterate
  - Prove value before scaling (evidence-based)
  - Learn from real usage (calibrate assumptions)
  - Scale only if successful (reduce risk)

Confidence: 50% overall
  - Phase 1: 80% (will work)
  - Phase 2: 60% (probably works)
  - Phase 3: 40% (many unknowns)

This is COMPLEX but DOABLE. Start small, prove value, scale carefully.
```

---

**Ready for Discussion**: YES  
**Next**: Await Codex + Gemini proposals, then discuss/reach consensus  
**If consensus**: Implement Phase 1 → Test → Measure → Iterate
