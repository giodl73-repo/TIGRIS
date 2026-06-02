# TIGRIS Trace

## Scope

Repo: TIGRIS

VTRACE stage: Trace

Baseline date: 2026-06-01

## Trace Matrix

| Trace ID | Mission / CONOPS | Requirements | Specs | Architecture | Interfaces | Verification | Validation | Status |
|---|---|---|---|---|---|---|---|---|
| TR-TIG-001 | NEED-TIG-001, CON-TIG-002 | REQ-TIG-001, REQ-TIG-008 | SPEC-TIG-001 | ARCH-TIG-001 | IF-TIG-001 | VER-TIG-001 | VAL-TIG-001 | traced_with_work_package_needed |
| TR-TIG-002 | NEED-TIG-003, CON-TIG-001 | REQ-TIG-002, REQ-TIG-003, REQ-TIG-004 | SPEC-TIG-002, SPEC-TIG-003 | ARCH-TIG-002 | IF-TIG-002 | VER-TIG-002 | VAL-TIG-002 | traced_with_work_package_needed |
| TR-TIG-003 | NEED-TIG-002, CON-TIG-003 | REQ-TIG-005 | SPEC-TIG-004 | ARCH-TIG-003 | IF-TIG-003 | VER-TIG-006 | VAL-TIG-003 | traced_with_work_package_needed |
| TR-TIG-004 | NEED-TIG-004, CON-TIG-004 | REQ-TIG-006 | SPEC-TIG-005 | ARCH-TIG-004 | IF-TIG-004 | VER-TIG-003, VER-TIG-004, VER-TIG-005 | VAL-TIG-004 | traced_with_work_package_needed |
| TR-TIG-005 | NEED-TIG-004, CON-TIG-005 | REQ-TIG-007 | SPEC-TIG-006 | ARCH-TIG-005 | IF-TIG-005 | VER-TIG-007 | VAL-TIG-005 | traced_with_work_package_needed |

## Open Trace Gaps

| Gap ID | Gap | Disposition |
|---|---|---|
| GAP-TIG-001 | Corpus/research claim checklist needs exact artifact rows. | Create work package for claim checklist lock. |
| GAP-TIG-002 | Simulator command ladder needs execution proof. | Create work package for simulator evidence packet. |
| GAP-TIG-003 | Shared-engine fixture schemas need package-level proof. | Create work package for fixture boundary evidence. |

Deferred specification visibility: SPEC-TIG-UNK-001, SPEC-TIG-UNK-002, and
SPEC-TIG-UNK-003 are intentionally dispositioned through later verification,
validation, interfaces, and work-package rows rather than treated as accepted
implementation specs.

## Role Review Summary

Role lenses applied from `.roles/`. No critical or major actionable trace gaps
remain outside the listed work-package candidates.
