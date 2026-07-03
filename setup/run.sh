#!/usr/bin/env bash
set -euo pipefail

log_file="run_$(date +%Y%m%d_%H%M%S).log"

uv --project ../Tractor-Crisp \
    run crisp safety-loop --llm-mode agent \
    2>&1 | tee -a "$log_file"
