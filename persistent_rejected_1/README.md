Resume the Codex session when the agent submits a change that then fails
validation. This gives the agent a chance to fix its mistake, rather than
rejecting the edit entirely and making the agent start fresh. Once a change is
accepted, we start a fresh session on the next turn of the loop.
