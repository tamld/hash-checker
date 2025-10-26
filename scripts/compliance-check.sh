#!/bin/bash
# Compliance Check Script
# Purpose: Enforce container-first approach and clean workspace
# Usage: ./scripts/compliance-check.sh [check|clean|pre-commit]

set -e

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_ROOT"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

check_workspace_clean() {
    log_info "Checking workspace cleanliness..."
    
    # Check for untracked files
    if [ -n "$(git status --porcelain | grep '^??')" ]; then
        log_error "Untracked files found:"
        git status --porcelain | grep '^??' | sed 's/^?? /  /'
        return 1
    fi
    
    # Check for modified files that shouldn't be modified
    if [ -n "$(git status --porcelain | grep '^ M')" ]; then
        log_warn "Modified files found:"
        git status --porcelain | grep '^ M' | sed 's/^ M /  /'
        log_warn "Review these changes before proceeding"
    fi
    
    log_info "✅ Workspace is clean"
    return 0
}

clean_workspace() {
    log_info "Cleaning workspace..."
    
    # Remove untracked files
    git clean -fd
    
    # Reset any uncommitted changes to core files
    git checkout -- rust/hash-checker-gui/src/main.rs 2>/dev/null || true
    
    log_info "✅ Workspace cleaned"
}

check_container_approach() {
    log_info "Checking container-first approach..."
    
    # Check if Docker is available
    if ! command -v docker &> /dev/null; then
        log_error "Docker is not available. Container-first approach requires Docker."
        return 1
    fi
    
    # Check if we're trying to run GUI tests on host
    if pgrep -f "hash-checker-gui.*--smoke-test" > /dev/null; then
        log_error "GUI tests are running on host. This violates container-first approach."
        log_error "Use 'make gui-automation-test' instead."
        return 1
    fi
    
    log_info "✅ Container-first approach verified"
    return 0
}

run_pre_commit_checks() {
    log_info "Running pre-commit checks..."
    
    # Check workspace cleanliness
    if ! check_workspace_clean; then
        log_error "Workspace is not clean. Run './scripts/compliance-check.sh clean' first."
        return 1
    fi
    
    # Check container approach
    if ! check_container_approach; then
        log_error "Container-first approach not followed."
        return 1
    fi
    
    # Run tests in container if automation image exists
    if docker image inspect hash-checker-gui-automation &> /dev/null; then
        log_info "Running tests in container..."
        make gui-automation-test
    else
        log_warn "GUI automation container not built. Run 'make gui-automation-build' first."
    fi
    
    log_info "✅ Pre-commit checks passed"
    return 0
}

show_help() {
    echo "Compliance Check Script"
    echo ""
    echo "Usage: $0 [command]"
    echo ""
    echo "Commands:"
    echo "  check       Check workspace cleanliness and container approach"
    echo "  clean       Clean workspace (remove untracked files)"
    echo "  pre-commit  Run full pre-commit checks"
    echo "  help        Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 check"
    echo "  $0 clean"
    echo "  $0 pre-commit"
}

main() {
    case "${1:-check}" in
        check)
            check_workspace_clean && check_container_approach
            ;;
        clean)
            clean_workspace
            ;;
        pre-commit)
            run_pre_commit_checks
            ;;
        help|--help|-h)
            show_help
            ;;
        *)
            log_error "Unknown command: $1"
            show_help
            exit 1
            ;;
    esac
}

main "$@"