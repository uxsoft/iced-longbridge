---
description: Release automation — on PR merge to master, bump workspace version, tag, create a GitHub release, and publish to crates.io.

on: pull_request merged

permissions:
  contents: read
  pull-requests: read
  issues: read

engine: copilot

runs-on: ubuntu-latest
timeout-minutes: 20

tools:
  bash:
    - "cargo read-manifest*"
    - "cargo metadata*"
    - "cargo tree*"
    - "git log*"
    - "git tag*"
    - "git show*"
    - "git diff*"
    - "git status"
    - "grep*"
    - "cat*"
    - "head*"
    - "tail*"
    - "ls*"
  github:
    allowed:
      - get_pull_request
      - list_commits
      - list_tags

safe-outputs:
  jobs:
    publish-release:
      description: "Bump the workspace version, build, tag, create a GitHub release, and publish `iced-longbridge` to crates.io."
      runs-on: ubuntu-latest
      permissions:
        contents: write
      inputs:
        bump:
          description: "semver bump level: major | minor | patch"
          required: true
          type: string
        notes:
          description: "Markdown release notes summarizing the merged PR(s) since the last tag."
          required: true
          type: string
      output: "Release published."
      steps:
        - uses: actions/checkout@v4
          with:
            ref: master
            fetch-depth: 0
            token: ${{ secrets.GITHUB_TOKEN }}

        - name: Extract request from agent output
          id: req
          run: |
            set -euo pipefail
            BUMP=$(jq -r '.items[] | select(.type == "publish_release") | .bump' "$GH_AW_AGENT_OUTPUT" | head -n1)
            NOTES=$(jq -r '.items[] | select(.type == "publish_release") | .notes' "$GH_AW_AGENT_OUTPUT" | head -n1)
            case "$BUMP" in major|minor|patch) ;; *) echo "invalid bump: $BUMP" >&2; exit 1 ;; esac
            {
              echo "bump=$BUMP"
              echo "notes<<__NOTES_EOF__"
              echo "$NOTES"
              echo "__NOTES_EOF__"
            } >> "$GITHUB_OUTPUT"

        - name: Compute new version
          id: version
          env:
            BUMP: ${{ steps.req.outputs.bump }}
          run: |
            set -euo pipefail
            CURRENT=$(grep -m1 -E '^version\s*=' Cargo.toml | sed -E 's/.*"([^"]+)".*/\1/')
            IFS='.' read -r MAJ MIN PAT <<<"$CURRENT"
            case "$BUMP" in
              major) MAJ=$((MAJ+1)); MIN=0; PAT=0 ;;
              minor) MIN=$((MIN+1)); PAT=0 ;;
              patch) PAT=$((PAT+1)) ;;
            esac
            NEW="$MAJ.$MIN.$PAT"
            echo "current=$CURRENT" >> "$GITHUB_OUTPUT"
            echo "new=$NEW" >> "$GITHUB_OUTPUT"
            echo "tag=v$NEW" >> "$GITHUB_OUTPUT"

        - name: Bump version in Cargo.toml
          env:
            NEW_VERSION: ${{ steps.version.outputs.new }}
          run: |
            set -euo pipefail
            python3 -c '
            import os, re, sys
            new = os.environ["NEW_VERSION"]
            if not re.fullmatch(r"\d+\.\d+\.\d+", new):
                sys.exit(f"bad version: {new}")
            src = open("Cargo.toml").read()
            src = re.sub(r"^version\s*=\s*\"[^\"]+\"", f"version = \"{new}\"", src, count=1, flags=re.M)
            open("Cargo.toml", "w").write(src)
            '
            cargo check --workspace

        - name: Build (release)
          run: cargo build --release --workspace

        - name: Commit and push version bump
          env:
            TAG: ${{ steps.version.outputs.tag }}
          run: |
            set -euo pipefail
            git config user.name  "github-actions[bot]"
            git config user.email "41898282+github-actions[bot]@users.noreply.github.com"
            git add Cargo.toml Cargo.lock
            git commit -m "chore(release): $TAG"
            git tag -a "$TAG" -m "Release $TAG"
            git push origin master
            git push origin "$TAG"

        - name: Create GitHub release
          env:
            GH_TOKEN: ${{ secrets.GITHUB_TOKEN }}
            TAG: ${{ steps.version.outputs.tag }}
            NOTES: ${{ steps.req.outputs.notes }}
          run: |
            set -euo pipefail
            printf '%s\n' "$NOTES" > /tmp/notes.md
            gh release create "$TAG" \
              --title "$TAG" \
              --notes-file /tmp/notes.md \
              --target master

        - name: Publish to crates.io
          env:
            CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}
          run: cargo publish -p iced-longbridge --token "$CARGO_REGISTRY_TOKEN"
---

# Release `iced-longbridge` on master merge

A pull request was merged to `master`:

- Number: `#${{ github.event.pull_request.number }}`
- Title: `${{ github.event.pull_request.title }}`

Your job is to decide the next semver bump and draft the release notes. You **do not** run the build, the tag, the release, or the publish — a downstream job called `publish-release` does all of that. You just call it with the right inputs.

## Steps

1. Read the current workspace version from `Cargo.toml` (under `[workspace.package]`).
2. Look at the last few tags: `git tag --list 'v*' --sort=-v:refname | head -n 5`.
3. Inspect the merged PR and any commits on `master` since the last tag to understand what changed.
4. Pick the right `bump`:
   - **major** — breaking API changes (body contains `BREAKING CHANGE`, or PR labeled `breaking`)
   - **minor** — new user-facing feature (title starts with `feat`, or PR labeled `feature`)
   - **patch** — fixes, docs, chores, internal refactors, dependency bumps
5. Draft concise Markdown release notes. Summarize the PR in 2–4 bullets (user impact, not file-by-file). Skip noise like formatting or CI-only changes. Don't include a "Full changelog" link — GitHub adds that automatically if we want it later.
6. Call the `publish-release` tool **exactly once** with:
   - `bump`: `major` | `minor` | `patch`
   - `notes`: your Markdown release notes

That's all. If you can't confidently pick a bump (e.g. empty or confusing PR description), default to `patch`.

## One-time setup (for the repo owner)

- Add a `CARGO_REGISTRY_TOKEN` repository secret (crates.io API token with `publish-update` scope):
  ```
  gh aw secrets set CARGO_REGISTRY_TOKEN
  ```
- Only `iced-longbridge` is published. `demo`, `demo-app`, and `demo-web` stay local.
