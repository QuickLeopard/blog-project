# Generate Git Commit Message

Run these commands first to gather context:
1. `git status` — see staged/unstaged files and branch
2. `git diff --staged` — staged changes; if output is empty, run `git diff` for unstaged changes instead; if both are empty, state that there is nothing to commit
3. `git log -5 --oneline` — see recent commit style for consistency

Based on the output, generate a commit message in standard git format:

**Format rules:**
- **First line (subject):** Imperative mood, 50 chars or less, no period at end
  - Example: `Add user authentication with JWT`
  - Use Conventional Commits when appropriate: `type(scope): description` (e.g., `feat(auth): add JWT login`)
  - Types: feat, fix, docs, style, refactor, test, chore
- **Blank line** between subject and body
- **Body (optional):** Explain *what* and *why*, wrap at ~72 chars
- **Footer (optional):** References like `Refs: #123`, co-authors

**Example output:**
```
feat(auth): add JWT-based login flow

- Add login endpoint with token generation
- Add AuthMiddleware for protected routes
- Update frontend to store and send Bearer token

Refs: #42
```

**Output:** Return only the raw commit message text, ready to paste into `git commit -m` or the commit message editor. No markdown code fence around it unless the user asks to copy it.
