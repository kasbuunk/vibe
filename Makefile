.PHONY: help cluster-up cluster-down cluster-status test clean build run fmt lint

help:
	@echo "Makefile targets:"
	@echo "  help          - Show this help message"
	@echo "  cluster-up   - Start the Kind cluster"
	@echo "  cluster-down - Stop the Kind cluster"
	@echo "  cluster-status - Show cluster status"
	@echo "  build        - Build the project"
	@echo "  run          - Run the project"
	@echo "  test         - Run tests"
	@echo "  fmt          - Format code"
	@echo "  lint         - Lint code"
	@echo "  clean        - Clean build artifacts"

# Cluster configuration
CLUSTER_NAME = vibe

# Project configuration
PROJECT_NAME = vibe

# Cluster management
cluster-up:
	@echo "Starting Kind cluster..."
	@kind delete cluster --name $(CLUSTER_NAME) || true
	@kind create cluster --name $(CLUSTER_NAME)
	@sleep 5
	@echo "Cluster status:"
	@kind get clusters

cluster-down:
	@echo "Stopping Kind cluster..."
	@kind delete cluster --name $(CLUSTER_NAME)

cluster-status:
	@echo "Cluster status:"
	@kind get clusters

# Build and run
build:
	@echo "Building project..."
	@cargo build

run:
	@echo "Running project..."
	@cargo run

test:
	@echo "Running tests..."
	@cargo test

# Code quality
clean:
	@echo "Cleaning build artifacts..."
	@cargo clean

fmt:
	@echo "Formatting code..."
	@cargo fmt

lint:
	@echo "Linting code..."
	@cargo clippy --all-targets --all-features -- -D warnings

# Development workflow
develop: fmt lint build test
	@echo "Development workflow completed successfully"

# Production workflow
prod: clean build
	@echo "Production build completed successfully"

# Docker
docker-build:
	@echo "Building Docker image..."
	docker build -t $(PROJECT_NAME) .

docker-push:
	@echo "Pushing Docker image..."
	docker push $(PROJECT_NAME)

# Kubernetes
k8s-deploy:
	@echo "Deploying to Kubernetes..."
	kubectl apply -f k8s/

k8s-delete:
	@echo "Deleting from Kubernetes..."
	kubectl delete -f k8s/

k8s-status:
	@echo "Kubernetes status:"
	kubectl get all
