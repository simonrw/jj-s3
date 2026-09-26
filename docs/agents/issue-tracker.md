# Issue tracker: GitHub

Issues and specs live in GitHub Issues for `simonrw/jj-s3`. Use the `gh` CLI with `--repo simonrw/jj-s3`.

## Operations

- Publish a spec with `gh issue create --repo simonrw/jj-s3 --title "..." --body-file <file> --label ready-for-agent`.
- Read an issue and its discussion with `gh issue view <number> --repo simonrw/jj-s3 --comments`. Fetch its labels with `--json labels` when needed.
- List issues with `gh issue list --repo simonrw/jj-s3 --state open --json number,title,body,labels`, adding label or state filters as needed.
- Apply or remove labels with `gh issue edit <number> --repo simonrw/jj-s3 --add-label "..."` or `--remove-label "..."`.
- When authorized to comment, use `gh issue comment <number> --repo simonrw/jj-s3 --body-file <file>`.
- Close completed issues with `gh issue close <number> --repo simonrw/jj-s3`.

Write multiline bodies to a file and pass `--body-file` to preserve Markdown and avoid shell interpolation.

## Pull requests as a request surface

PRs as a request surface: no.

GitHub shares issue and pull request numbers. For an ambiguous reference, try `gh pr view <number>` and then `gh issue view <number>`.
