# Contributing to bioxus

bioxus is public so that anyone can read it, learn from it and help build it. It is
licensed under the PolyForm Noncommercial License 1.0.0 (see `LICENSE`): free for
noncommercial use, commercial use needs the written permission of the owner.

## Before you send code

1. Open an issue first for anything bigger than a fix, so the direction is agreed.
2. Follow `CLAUDE.md` (conventions) and run `make ci-local`: format, clippy with
   `-D warnings`, wasm check and tests must pass. Each public widget has a test.
3. Keep bioxus a library, not a framework: no imposed router, state, language or look;
   user-facing text is always a prop.

## Contributor agreement

By opening a pull request you confirm that:

- you wrote the contribution, or have the right to submit it;
- you grant Baptiste Grosjean a perpetual, worldwide, irrevocable, royalty-free
  licence to use, copy, modify, distribute and sublicense your contribution, under
  any licence, including a commercial or different one, and you keep your own
  copyright;
- you sign off every commit (`git commit -s`), which states the above for that commit.

A pull request without sign-off is not merged. This text is a plain-language
agreement, not legal advice; the owner may replace it with a formal CLA, and will
ask for your agreement again if so.
