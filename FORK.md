# Fork maintenance

Upstream: https://github.com/ulyssa/iamb
Fork: https://github.com/mi-skam/iamb
Feature branch: `feat/message-history-recall`
Initial upstream base: `v0.0.12` (`b268cc7`).

This branch adds composer input-history recall with Up/Down in Insert mode,
Shift-Up/Shift-Down for multiline draft navigation, and recording of successfully
queued text input. Normal-mode arrow behavior is unchanged. History is held in
memory per chat view; it is not persisted across application restarts.

## Remotes

```sh
git remote -v
# origin    https://github.com/mi-skam/iamb.git
# upstream  https://github.com/ulyssa/iamb.git
```

## Rebase onto a new upstream release

Start with a clean working tree. Replace the example NEW_BASE with an actual
published upstream release, and set OLD_BASE to the release currently used by
this branch. After each successful rebase, update the current-base record below.

```sh
git fetch upstream --tags
git fetch origin
git switch feat/message-history-recall
git status --short

OLD_BASE=v0.0.12
NEW_BASE=v0.0.13  # example only: verify that this release exists first

git branch "backup/message-history-recall-before-${NEW_BASE}"
git rebase --onto "$NEW_BASE" "$OLD_BASE" feat/message-history-recall

# Resolve conflicts and use `git rebase --continue`, or cancel with:
# git rebase --abort

cargo test --release --locked
cargo build --release --locked

# Test the built binary interactively before publishing/installing it:
# ./target/release/iamb

# Rebasing rewrites branch history. Inspect before pushing, and coordinate
# with anyone else using this branch. Do not force-push upstream or main.
git diff "$NEW_BASE"...HEAD
git push --force-with-lease origin feat/message-history-recall
```

If upstream adds equivalent functionality, compare it with this patch and drop
obsolete changes rather than carrying duplicate bindings or history recording.

Current upstream base: `v0.0.12` (`b268cc7`).

## Interactive checks

- Send a text message; in Insert mode, Up recalls it without sending anything.
- Repeated Up/Down traverses input history and restores the pending draft.
- Recall works immediately after entering Insert mode from message history.
- Shift-Up/Shift-Down move between multiline draft lines in Insert mode.
- Normal-mode arrows still navigate messages.
- Message-send errors do not add failed input to history.
- Externally edited input and successful edits of queued local messages are
  recorded as input, preserving slash-command and Markdown syntax.

## Installing locally

Exit iamb, back up the installed executable, then install the tested build:

```sh
cp -p ~/.local/bin/iamb ~/.local/bin/iamb.before-update
install -m 755 target/release/iamb ~/.local/bin/iamb
```

The normal version banner identifies the upstream commit, not the fork's
feature name; use `git rev-parse HEAD` when recording the exact source revision.
