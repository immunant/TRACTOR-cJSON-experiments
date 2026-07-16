#!/usr/bin/env bash
set -euo pipefail

# Commit the initial set of files.
uv run --project ../Tractor-Crisp \
    crisp commit -t c_code \
    cJSON.c cJSON.h CMakeLists.txt library_config tests test_translated.sh

# Run `crisp main` through the deterministic setup stages, then stop before the
# first LLM/agent safety rewrite. The safety step consumes fuel before invoking
# the LLM, so zero safety tries exits without starting the AI portion.
#
# It would be nice if we had something like `crisp transpile` that could do this
# more directly, setting safety tries to 0 feels like a hack.
LLM_SAFETY_TRIES=0 \
uv run --project ../Tractor-Crisp crisp main --llm-mode agent
