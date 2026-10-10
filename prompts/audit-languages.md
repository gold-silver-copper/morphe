# Audit morphe languages (YOLO mode)

You're auditing language configs in **morphe** (`~/code/morphe`). Read
`prompts/add-languages.md` first: it has the commands, rules and options.
YOLO mode: no tests and no evaluation harness. You look at real output with
`morphe show`/`morphe analyze`, fix what's wrong, rebuild, retrain, push.

For each language assigned to you:
1. `./target/release/morphe show <lang> <pos> <word>` for 4–6 common real
   words per main part of speech (nouns, verbs, adjectives), plus 2 made-up
   words. Also run `morphe analyze <lang> <form>` on 3 inflected forms.
2. Look for systematic problems:
   - leftover notes, parentheses, digits, slashes, or pronouns/labels
     inside forms;
   - cells that are always empty or that duplicate another cell;
   - forms filed in the wrong cell, or a cell missing its main variant;
   - spelling or encoding junk (mixed scripts, doubled marks, marks on the
     wrong letters);
   - made-up words getting absurd forms;
   - inherent features (gender, aspect…) missing or wrong;
   - the `samples()` hook missing: add 6–8 real and made-up words for the
     playground.
3. Fix the config (`configs/src/langs/<lang>.rs`) with existing options or
   hooks: `retag`, `prepare`, `reshape`, `split_form`, `strip_words`,
   `drop_tags`, `soft_tags`, `lemma_cell`, `first_table_only`… Generic
   trainer improvements are allowed only if they are additive and contain
   no language names.
4. `morphe train <lang>`, re-check, run `cargo build --release -j 4` (whole
   workspace), commit only that language's files, then
   `git pull --rebase origin main`, rebuild and retrain if the pull changed
   core code, and push. One language at a time.

Rules:
- Never write to /tmp.
- `-j 4` max.
- Don't change the runtime model format.
- Don't touch languages not assigned to you; other agents are editing
  them.
- At the end, remove your worktree, kill anything you started, and report
  in a few lines per language: what was wrong and what you fixed.
