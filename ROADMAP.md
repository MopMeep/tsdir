# tsdir Roadmap

## Current Version: 0.4.0

---

## ✅ Implemented

- [x] Create timestamped directories: `tsdir name` → `name-YYYY-MM-DD-HH-MM`
- [x] Optional name: `tsdir` → `YYYY-MM-DD-HH-MM`
- [x] Verbosity levels: `-V 1` (date), `-V 2` (date+time, default), `-V 3` (date+time+seconds)
- [x] Parent directory: `-p/--parent DIR`
- [x] Dry run: `-d/--dry-run`
- [x] `mkdir`-like behavior: fails if exists by default
- [x] `-P/--parents` flag for `mkdir -p` behavior (create parents, idempotent)

---

## 🎯 High Priority

### 1. Collision Handling (`--unique`)
Auto-append counter when directory exists:
```bash
tsdir --unique my-exp     # Creates my-exp-2026-11-23_2 if my-exp-2026-11-23 exists
tsdir -u my-exp           # Short form
```
**Reference:** maestro-core's `ensure_unique()` appends `_2`, `_3`, etc.

### 2. Name Sanitization
Auto-clean input names (slugify):
```bash
tsdir "My Experiment!"    # → my-experiment-2026-11-23
tsdir "Test@#$%^&*()"     # → test-2026-11-23
```
**Reference:** maestro-core's `slugify()` - lowercase, collapse dashes, trim, cap at 50 chars.

---

## 🔧 Medium Priority

### 3. Force Overwrite (`-f/--force`)
Delete existing directory and recreate:
```bash
tsdir -f my-exp           # Removes existing, creates fresh
tsdir --force my-exp
```
**Reference:** mkdir2's `--force` flag.

### 4. Compact/Sortable Timestamp Formats
```bash
tsdir --compact           # 20261123-143045 (sortable, no separators)
tsdir --iso               # 2026-11-23T14:30:45Z (ISO 8601)
tsdir --unix              # 1795425481 (Unix timestamp)
```
**Reference:** canic-cli uses `YYYYMMDD-HHMMSS`; maestro-core uses `{slug}_{unix_ts}`.

### 5. Verbose Output (`-v/--verbose`)
Print each action:
```bash
tsdir -v my-exp           # "Created: ./my-exp-2026-11-23-14-30"
tsdir -v -P -p /tmp/sandbox my-exp
```
**Reference:** GNU mkdir `-v`, mkdir2 `--verbose`.

---

## 💡 Low Priority / Nice to Have

### 6. Batch Creation from File
```bash
tsdir --from experiments.txt
```
Each line = one directory name (supports comments, blank lines).
**Reference:** mkdir2's `--from` flag.

### 7. Gitkeep File
```bash
tsdir --gitkeep my-exp    # Creates .gitkeep inside
```
**Reference:** mkdir2's `--gitkeep` flag.

### 8. JSON Output
```bash
tsdir --json my-exp       # Machine-readable output
```
**Reference:** mkdir2's `--json` flag.

### 9. Permissions (`-m/--mode`)
```bash
tsdir -m 0750 my-exp      # Set directory permissions (Unix)
```
**Reference:** GNU mkdir `-m`, mkdir2 `-m`.

### 10. Interactive Mode
```bash
tsdir -i my-exp           # Prompt before overwrite
```
**Reference:** mkdir2's `--interactive`.

### 11. Stats/Summary
```bash
tsdir --stats my-exp1 my-exp2  # "Created: 2, Skipped: 0, Failed: 0"
```
**Reference:** mkdir2's `--stats` flag.

---

## 📝 Notes

- Keep CLI simple - prefer flags over complex subcommands
- Maintain `mkdir` compatibility where sensible
- Default behavior should be safe (fail on collision)
- All new features opt-in via flags
- Consider adding bash/zsh completion generation

---

## 🔗 References

- [cli-contract](https://docs.rs/cli-contract) - Path contract best practices
- [mkdir2](https://docs.rs/mkdir2) - Modern mkdir replacement
- [maestro-core run_dir](https://docs.rs/maestro-core) - Slugify + unique naming
- [canic-cli](https://docs.rs/canic-cli) - Compact timestamp format
- [GNU mkdir](https://www.gnu.org/software/coreutils/manual/html_node/mkdir-invocation.html) - Reference implementation