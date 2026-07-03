#!/usr/bin/env bash
set -euo pipefail

log_file="run_$(date +%Y%m%d_%H%M%S).log"

# HACK: Manually set the API endpoint for CRISP. This should probably just be
# the default when CRISP_API_KEY is set.
export CRISP_API_BASE=https://api.openai.com/v1

uv --project ../Tractor-Crisp \
    run crisp safety-loop --llm-mode agent \
    --resume-codex-session=rejected \
    2>&1 | tee -a "$log_file"
