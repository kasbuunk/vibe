#!/bin/bash

# Check if GitHub CLI is installed
git --version &> /dev/null
if [ $? -ne 0 ]; then
    echo "GitHub CLI is not installed. Please install it first."
    exit 1
fi

# Get detailed workflow run information
run_info=$(gh run list --limit 1 --json status,conclusion,updatedAt,workflowName --jq '.[0]')
status=$(echo $run_info | jq -r '.status')
conclusion=$(echo $run_info | jq -r '.conclusion')
updated_at=$(echo $run_info | jq -r '.updatedAt')
workflow_name=$(echo $run_info | jq -r '.workflowName')

# Print detailed status information
echo "Workflow: $workflow_name"
echo "Status: $status"
echo "Conclusion: $conclusion"
echo "Last updated: $updated_at"

# Check if there are any active runs
if [ "$status" = "in_progress" ]; then
    echo "\nActive workflow run in progress"
fi

# Get any failed jobs if the run failed
if [ "$conclusion" = "failure" ]; then
    echo "\nFailed jobs:"
    gh run list --limit 1 --json jobs --jq '.[0].jobs[] | select(.conclusion == "failure") | "\(.name) - \(.conclusion)"'
fi
