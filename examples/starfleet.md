---
title: "USS Enterprise Tech Brief"
subtitle: "Markdown Elements Guide"
logo: "resources/starfleet.svg"
author: "Lt. Commander Data"
date: "2024-12-20"
version: "NCC-1701-D"
language: "en"
toc: false
tags: ["starfleet", "technical", "markdown"]
participants: ["Jean-Luc Picard", "William Riker", "Data"]
template: "simple"
---

# USS Enterprise Tech Brief

A concise guide to markdown elements aboard the Federation flagship.

## Core Systems

### Warp Drive

The **warp core** maintains peak efficiency while _dilithium crystals_ power our journey. The ~~old conduits~~ have been upgraded.

#### Systems Status

- Warp coils aligned
- Antimatter stable
  - Primary containment
  - Backup protocols

#### Emergency Steps

1. Initiate shutdown
2. Eject core if needed
3. Switch to impulse

## Engineering Code

Ship's computer Rust function:

```rust
fn safe_warp_factor(integrity: f64) -> f64 {
    if integrity > 0.95 { 9.6 } else { 1.0 }
}
```

Use `warp_core::engage()` for FTL travel;

### Tactical Status

| System  | Status | Power |
| ------- | ------ | ----- |
| Phasers | Online | 100%  |
| Shields | Raised | 98%   |

## Mission Protocols

> "To boldly go where no one has gone before."
> — Starfleet Charter

### Away Team Checklist

- [x] Phaser
- [x] Tricorder
- [ ] Environmental suit

Key equations: $v = wf^3c$, use `scan_freq = 2.4_GHz`

---

**Live long and prosper.** 🖖

_Starfleet Command • Stardate 47391.2_
