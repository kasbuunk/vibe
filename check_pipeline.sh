#!/bin/bash

# Check if GitHub CLI is installed
git --version &> /dev/null
if [ $? -ne 0 ]; then
    echo "GitHub CLI is not installed. Please install it first."
    exit 1
fi

# Get the latest workflow run status
status=$(gh run list --limit 1 --json conclusion --jq '.[0].conclusion')

# Print the status
echo "Latest workflow status: $status"

# Check if there are any active runs
active_runs=$(gh run list --limit 1 --json status --jq '.[0].status')
if [ "$active_runs" = "in_progress" ]; then
    echo "There is an active workflow run in progress"
fi
