# VerityDB Dev Log

Append-only. Newest entry at the bottom. Never edit a past entry — if you turn out to have been
wrong, say so in a new one. The record of being wrong is the most valuable thing in this file.

## How to use it

**Write during the session or in the five minutes after it.** Not "later tonight", not "on the
weekend". A log written from memory three days later contains only the outcome, and the outcome
is the least interesting part.

**Record confusion, not just progress.** The entries that become posts are the ones where you
write "I don't understand why this works." Everyone who reads your eventual post was confused by
the same thing, and by the time you understand it you will have forgotten that it was ever
confusing. That forgetting is why most engineers can't write — you are capturing the thing that
gets lost.

**Record dead ends and wrong turns.** The instinct is to log the version that worked. The
approach you abandoned and why you abandoned it is better material and better thinking.

**Always fill in the last field.** "Next session starts by ___" is the highest-value line in the
template. The dominant cost of part-time work is reloading context, and deciding tomorrow's
first move while the context is still hot saves twenty minutes every single session.

**Numbers, however rough.** Even "roughly 40k writes/sec, no methodology, don't trust it" is
worth more later than no number at all. You cannot reconstruct a measurement you didn't take.

**Tag entries** so you can find material later without rereading everything. Suggested tags:
`#wal #fsync #sstable #compaction #mvcc #bitemporal #benchmark #crash #rust-borrow #confusion
#deadend #idea`.

**Monthly, on the first of the month:** skim the month's entries, pull the three best
confusions or dead ends into a `#post` list at the bottom of the file. That list is your
publishing queue, and it means you never write a post from a blank page — you write it from
notes you took while it was hard.

## Scope

This file is technical and belongs in the public repo. Keep Track B material — commercial
thinking, notes on conversations, anything touching your current employer — in a separate
private file. Do not mix them.

## Template

```
## NNN — YYYY-MM-DD — vX.Y — Nh
Goal:
Did:
Broke / confused me:
Learned:
Numbers:
Open questions:
Next session starts by:
Tags:
```

---

## 001 — 2026-09-02 — v0.1 — 0.5h
Goal: Setup veritydb. Write a small key value store. 
Did: Started this new project. Did the in memory key value store.
Broke / confused me: String vs str. Why one has default value the other does not. The
reference `&` thing and `mut` thing also confused me.
Learned: Basic rust syntax. Functions like println!, print!, etc
Numbers: None.
Open questions: How does `mut` actually work? How does `&` work? Do we have classes in rust?
Next session starts by: Setting up a proper rust project with `cargo`.
Tags: #rust-borrow #confusion #setup
