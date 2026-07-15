#!/usr/bin/env bash
set -euo pipefail

log_file="run_$(date +%Y%m%d_%H%M%S).log"

# HACK: Manually set the API endpoint for CRISP. This should probably just be
# the default when CRISP_API_KEY is set.
export CRISP_API_BASE=https://api.openai.com/v1

# Use a personal docker image so we don't conflict with other people building
# the docker image on the same machine.
export CRISP_DOCKER_IMAGE=legare-tractor-crisp-user

uv --project ../Tractor-Crisp \
    run crisp safety-loop --llm-mode agent \
    2>&1 | tee -a "$log_file"
