### **AGENTS.MD**

**Agent Name:** Jules
**Mission:** Systematically implement the `onebox-rs` internet bonding solution according to the provided project documentation.

#### **1. Core Directives**

1.  **Specification-Driven:** Your primary source of truth is the [Software Requirements Specification (SRS.md)](./docs/SRS.md). All implementation details must be traceable to a requirement in this document. The [Product Requirements Document (PRD.md)](./docs/PRD.md) provides overall project context.
2.  **Task-Oriented Workflow:** You will work by sequentially processing tasks from the [TASKS.md](./docs/TASKS.md). You will only proceed to the next task upon receiving the command `next task`.
3.  **Test-After-Implement:** After completing each task, you must immediately consult the [TEST_PLAN.md](./docs/TEST_PLAN.md) and execute the relevant test scenarios to validate your implementation. A task is not "Done" until its corresponding tests pass.
4.  **Idiomatic Rust:** All code must be safe, concurrent, and idiomatic Rust. It must pass `cargo fmt` and `clippy --deny warnings` at all times.
5.  **Document Continuously:** As you implement functions and modules, provide clear, concise doc comments explaining their purpose, parameters, and return values.

#### **2. Project Artifacts**

*   **[PRD.md](./docs/PRD.md):** The "Why." The high-level vision, goals, and features of the project.
*   **[SRS.md](./docs/SRS.md):** The "What." The detailed technical specification of every function and requirement. **This is your blueprint.**
*   **[TEST_PLAN.md](./docs/TEST_PLAN.md):** The "How to Verify." The specific scenarios and steps to validate the implementation against the SRS.
*   **[TASKS.md](./docs/TASKS.md):** The "How to Build." Your ordered checklist of implementation steps organized by phases.
*   **[CHANGELOG.md](./changelog/CHANGELOG.md):** The "What's New." Complete history of changes, organized by version.

#### **3. Development Workflow**

You will operate in a stateful loop, initiated by the `next task` command.

1.  **Acknowledge Command:** Upon receiving `next task`, acknowledge the command.
2.  **Consult Task List:** Read `docs/TASKS.md` and identify the first task with the status `To Do`.
3.  **Update Status:** Modify `docs/TASKS.md` in-place, changing the status of the selected task to `In Progress`.
4.  **Announce Task:** State the ID, description, and phase of the task you are about to perform.
5.  **Reference & Implement:**
    *   Reference the relevant section(s) from `docs/SRS.md`.
    *   Implement the required Rust code, creating or modifying files as needed.
    *   Follow the phase-based approach: complete all tasks in a phase before moving to the next.
6.  **Reference & Test:**
    *   Reference the relevant test case ID(s) from `docs/TEST_PLAN.md`.
    *   Describe how you will perform the test (e.g., "I will now run the client and server and execute `ping 8.8.8.8` to validate TS1.1.").
    *   Execute the test.
7.  **Conclude Task:**
    *   **On Success:** If the test passes, announce success. Modify `docs/TASKS.md`, changing the task's status to `Done`.
    *   **On Failure:** If the test fails, announce the failure and the reason. Enter a "Debugging" state to fix the implementation. Do not change the task status until the test passes.
8.  **Update Changelog:** After successfully completing a task, update the changelog system:
    *   Add the completed task to `changelog/UNRELEASED.md` under the appropriate section
    *   If this completes a phase or major milestone, consider creating a new version entry
    *   Ensure all changelog entries are properly categorized (Added, Changed, Fixed, etc.)
9.  **Phase Completion Check:** After completing a task, check if all tasks in the current phase are done. If so, announce phase completion.
10. **Await Next Command:** Announce that you are ready for the next instruction.

#### **4. Task Phases & Dependencies**

The implementation is organized into 8 logical phases:

- **Phase 1: Project Foundation & Core Infrastructure** (T0-T3)
- **Phase 2: Basic Networking & TUN Interface** (T4-T7) 
- **Phase 3: Core Bonding Engine** (T8-T11)
- **Phase 4: Security & Authentication** (T12-T13)
- **Phase 5: Link Health & Failover** (T14-T16)
- **Phase 6: Performance & Optimization** (T17-T19)
- **Phase 7: Testing & Quality Assurance** (T20-T24)
- **Phase 8: Documentation & Deployment** (T25-T28)

**Critical Path:** T0 → T1 → T4-T7 → T8 → T9-T11 → T12-T13 → T20-T24 → T28

#### **5. Implementation Guidelines**

- **Start with T0 (Project Scaffolding):** This is the foundation for everything else
- **Complete phases sequentially:** Don't skip ahead to later phases until current phase is complete
- **Test incrementally:** Each task should be tested as soon as it's implemented
- **Maintain code quality:** All code must pass formatting and linting checks
- **Document as you go:** Add clear doc comments and update relevant documentation
- **Maintain changelog:** Update changelog entries for all completed tasks and milestones

#### **6. Success Criteria**

A task is considered **Done** when:
1. ✅ Implementation is complete according to SRS requirements
2. ✅ Code passes `cargo fmt` and `clippy --deny warnings`
3. ✅ All related tests pass
4. ✅ Documentation is updated
5. ✅ Changelog is updated with completed task
6. ✅ Code review is completed (if applicable)

**Initial State:** Awaiting `next task` to begin with **T0: Project Scaffolding** in Phase 1.

#### **7. Documentation Standards**

- **Primary Diagrams:** The main `README.md` should use Mermaid JS for all embedded diagrams to ensure they render correctly on GitHub.
- **Alternative Formats:** To support a wider range of tooling, alternative versions of diagrams are encouraged.
  - **D2:** D2 source files (`.d2`) should be stored in `docs/diagrams/d2/`.
  - **PlantUML:** PlantUML source files (`.puml`) should be stored in `docs/diagrams/plantuml/`. A `README.md` within this directory should provide a human-readable view of the diagrams.
- **Animation POCs:** Proofs-of-concept for animated diagrams (Manim, anime.js, JointJS, X6 AntV, three.js, etc.) should be stored in their own subdirectories within `docs/diagrams/`. Rendered output (videos, GIFs) should not be committed to the repository.

---
### **docs/PRD.md**

*(This file contains the full Product Requirements Document created in the previous step.)*

---
### **docs/SRS.md**

*(This file contains the full Software Requirements Specification created in the previous step.)*

---
### **docs/TEST_PLAN.md**

*(This file contains the full Test Plan & Scenarios created in the previous step.)*

---
### **docs/TASKS.md**

*(This file contains the comprehensive task list organized by implementation phases.)*