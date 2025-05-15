#!/bin/bash

# Set script to exit on any error
set -e

# Function to create cluster
create_cluster() {
    echo "Creating Kind cluster..."
    kind create cluster --name vibe
}

# Function to delete cluster
delete_cluster() {
    echo "Deleting Kind cluster..."
    kind delete cluster --name vibe
}

# Main script logic
# First try to delete any existing cluster
delete_cluster || true  # Don't fail if cluster doesn't exist

# Create new cluster
create_cluster

# Wait a bit for cluster to be ready
sleep 5

# Verify cluster creation
echo "Cluster status:"
kind get clusters
