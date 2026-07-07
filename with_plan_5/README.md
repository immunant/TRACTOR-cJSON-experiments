- Normal agent run with persistent safety plan and fresh agent sessions every
  turn of the loop.
- 3rd attempt at a fix for `cargo check-unsafe2`, this time we're using a fresh
  target dir every time so there should be no further conflicts with the agent
  running `cargo build`.
- Give the agent a way to diff its pending changes against a baseline, since it
  keeps trying and failing to use `git diff`.
- Tell the agent how to query the unsafe JSON.
