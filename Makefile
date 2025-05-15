.PHONY: help cluster-up cluster-down cluster-status test clean

help:
	@echo "Makefile targets:"
	@echo "  help          - Show this help message"
	@echo "  cluster-up   - Start the Kind cluster"
	@echo "  cluster-down - Stop the Kind cluster"
	@echo "  cluster-status - Show cluster status"
	@echo "  test         - Run tests"
	@echo "  clean        - Clean build artifacts"

CLUSTER_NAME = vibe

cluster-up:
	@echo "Starting Kind cluster..."
	@kind delete cluster --name $(CLUSTER_NAME) || true
	@kind create cluster --name $(CLUSTER_NAME)
	@sleep 5
	@kind get clusters

cluster-down:
	@echo "Stopping Kind cluster..."
	@kind delete cluster --name $(CLUSTER_NAME)

cluster-status:
	@echo "Cluster status:"
	@kind get clusters

# Add more targets as needed
# test:
# 	@cargo test

# clean:
# 	@cargo clean
