[![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.18682846.svg)](https://doi.org/10.5281/zenodo.18682846)

# pmatrix-encoder

**Reference encoder for schema conformance. Not an execution engine.**

A command-line tool that emits and validates P-MATRIX Runtime State records
conforming to the [Runtime State Schema v1.0.0](https://pmatrix.io/schema/runtime-state/1.0.0).

## ⚠️ Important Notice

This implementation is for schema conformance demonstration only.
It does NOT reflect any production, kernel, or normative logic of P-MATRIX.
Production implementations use proprietary evaluation pipelines that are
fundamentally different from this demonstration logic.

## Usage

### Emit a demonstration record

```bash
pmatrix-encoder emit \
  --baseline 0.25 \
  --norm 0.70 \
  --stability 0.30 \
  --meta-control 0.20
```

Output:
```json
{
  "spec_version": "pmatrix-3.5",
  "schema_version": "1.0.0",
  "timestamp": 1707500000,
  "functions": {
    "baseline": 0.25,
    "norm": 0.70,
    "stability": 0.30,
    "meta_control": 0.20
  },
  "stability_score": 0.3625,
  "risk_score": 0.6375,
  "mode": "Alert",
  "risk_level": "L4"
}
```

### Validate a record

```bash
echo '{ ... }' | pmatrix-encoder validate
```

Output:
```
[PASS] INV-R1 — All function values in [0.0, 1.0].
[PASS] INV-R2 — stability_score=0.3625
[PASS] INV-R3 — risk_score=0.6375
...
Result: ALL INVARIANTS SATISFIED — record is conforming.
```

## Invariants Checked

All 12 invariants from the P-MATRIX Runtime State Schema v1.0.0:

| ID | Description |
|----|-------------|
| INV-R1 | All function values in [0.0, 1.0] |
| INV-R2 | stability_score in [0.0, 1.0] |
| INV-R3 | risk_score in [0.0, 1.0] |
| INV-R4 | timestamp > 0 |
| INV-C1 | mode = threshold_map(risk_score) |
| INV-C2 | risk_level = level_map(mode) |
| INV-C3 | mode and risk_level mutually consistent |
| INV-S1 | All eight required fields present |
| INV-S2 | No additional fields |
| INV-S3 | spec_version = "pmatrix-3.5" |
| INV-S4 | schema_version is valid semver |
| INV-T1 | Timestamps monotonically non-decreasing (stream-level) |

## License

Apache-2.0

---

*P-MATRIX Runtime State Representation — pmatrix-3.5*
*Patent Pending: KR Application No. 10-2025-0216047*
