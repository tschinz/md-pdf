---
type: "report"
title: "Quarterly Engineering Report"
description: "Q3 2026 — progress, metrics, and roadmap"
logo: "resources/md-pdf.svg"
author: "Silvan Zahno"
timestamp: "2026-09-24"
version: "1.0.0"
status: "final"
language: "en"
toc: true
tags: ["engineering", "quarterly", "report"]
participants: ["Silvan Zahno", "Jane Rivera", "Data"]
template: "professional"
---

# Executive Summary

This report summarizes engineering progress for the third quarter of 2026. The
team shipped the OKF-compatible front matter, image sizing, and a redesigned
template system, while keeping the build green and the dependency tree audit-clean.

# Highlights

- **OKF front matter** — canonical field names with backwards-compatible aliases.
- **Image control** — width, height, and alignment via the HTML `<img>` tag.
- **Templates** — a document `type` label and two new templates (`professional`, `letter`).

# Metrics

| Metric              | Q2 2026 | Q3 2026 | Change |
| ------------------- | ------: | ------: | -----: |
| Templates shipped   |       5 |       7 |     +2 |
| Open vulnerabilities|       4 |       0 |     -4 |
| Tests               |      12 |      16 |     +4 |

# A Small Formula

The compression ratio is estimated as:

$$ r = 1 - \frac{s_{compressed}}{s_{original}} $$

# Roadmap

1. Surface `status` and `resource` in the document templates.
2. Add multi-author support to the front matter.
3. Expand the template gallery.

# Conclusion

The quarter delivered a breaking but well-scoped `0.2.0` release. The next
quarter focuses on richer metadata rendering and broader template coverage.
