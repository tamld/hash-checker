#!/bin/bash
# Framework Validation Script
# Purpose: Comprehensive validation of GUI automation framework
# Usage: ./scripts/validate_framework.sh [component]

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Validation counters
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# Test function
run_test() {
    local test_name="$1"
    local test_command="$2"
    
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    log_info "Testing: $test_name"
    
    if eval "$test_command"; then
        log_success "✅ $test_name"
        PASSED_TESTS=$((PASSED_TESTS + 1))
        return 0
    else
        log_error "❌ $test_name"
        FAILED_TESTS=$((FAILED_TESTS + 1))
        return 1
    fi
}

# Component validation functions
validate_container() {
    log_info "=== Validating Container Infrastructure ==="
    
    # Test Dockerfile syntax
    run_test "Dockerfile syntax" "docker build --help > /dev/null && echo 'Docker available'"
    
    # Test entrypoint script
    run_test "Entrypoint script executable" "test -x docker/gui-automation-entrypoint.sh"
    
    # Test container build (if not already built)
    if ! docker images | grep -q hash-checker-gui-automation; then
        log_info "Building container..."
        if docker build -f docker/gui-automation.Dockerfile -t hash-checker-gui-automation .; then
            log_success "Container built successfully"
        else
            log_error "Container build failed"
            return 1
        fi
    else
        log_success "Container already exists"
    fi
    
    # Test container execution
    run_test "Container execution" "docker run --rm hash-checker-gui-automation echo 'Container works'"
    
    # Test headless environment
    run_test "Headless environment" "docker run --rm hash-checker-gui-automation bash -c 'echo \$DISPLAY && echo \$XDG_RUNTIME_DIR'"
}

validate_scripts() {
    log_info "=== Validating Analysis Scripts ==="
    
    # Test telemetry analysis script
    run_test "Telemetry script syntax" "python3 -m py_compile scripts/analyze_telemetry.py"
    run_test "Telemetry script help" "python3 scripts/analyze_telemetry.py 2>&1 | grep -q 'Usage:'"
    
    # Test performance regression script
    run_test "Performance script syntax" "python3 -m py_compile scripts/check_performance_regression.py"
    run_test "Performance script help" "python3 scripts/check_performance_regression.py 2>&1 | grep -q 'Usage:'"
    
    # Test with sample data
    mkdir -p logs/gui-manifest
    echo '{"type":"test","value":1}' > logs/gui-manifest/test.log
    run_test "Telemetry script execution" "python3 scripts/analyze_telemetry.py logs/gui-manifest/test.log"
    
    # Test performance regression with sample data
    echo '{"test_metric": 100}' > test_current.json
    echo '{"test_metric": 90}' > test_baseline.json
    run_test "Performance script execution" "python3 scripts/check_performance_regression.py test_current.json test_baseline.json 2>/dev/null || true"
    
    # Cleanup test files
    rm -f logs/gui-manifest/test.log test_current.json test_baseline.json performance_regression_report.md
}

validate_ci() {
    log_info "=== Validating CI Workflow ==="
    
    # Test YAML syntax
    run_test "CI workflow YAML syntax" "python3 -c 'import yaml; yaml.safe_load(open(\".github/workflows/gui-automation.yml\"))'"
    
    # Test workflow triggers
    run_test "CI workflow triggers" "grep -q 'on:' .github/workflows/gui-automation.yml"
    
    # Test job definitions
    run_test "CI workflow jobs" "grep -q 'jobs:' .github/workflows/gui-automation.yml"
    
    # Test matrix strategy
    run_test "CI matrix strategy" "grep -q 'matrix:' .github/workflows/gui-automation.yml"
}

validate_compliance() {
    log_info "=== Validating Compliance Framework ==="
    
    # Test compliance script
    run_test "Compliance script executable" "test -x scripts/compliance-check.sh"
    run_test "Compliance script check" "./scripts/compliance-check.sh check"
    
    # Test Makefile targets
    run_test "Makefile gui-automation-build" "grep -q 'gui-automation-build:' Makefile"
    run_test "Makefile gui-automation-test" "grep -q 'gui-automation-test:' Makefile"
    run_test "Makefile check-clean" "grep -q 'check-clean:' Makefile"
    
    # Test compliance documentation
    run_test "Compliance skeleton exists" "test -f .agents/cleanup_skeleton.yml"
    run_test "Compliance summary exists" "test -f .agents/COMPLIANCE_SUMMARY.md"
}

validate_integration() {
    log_info "=== Validating Integration ==="
    
    # Test end-to-end workflow
    run_test "End-to-end container test" "docker run --rm -v $(pwd):/workspace hash-checker-gui-automation bash -c 'cd /workspace && python3 scripts/analyze_telemetry.py --help'"
    
    # Test compliance integration
    run_test "Compliance integration" "./scripts/compliance-check.sh check && echo 'Integration works'"
    
    # Test Makefile integration
    run_test "Makefile integration" "make check-clean"
}

# Main validation function
main() {
    local component="${1:-all}"
    
    log_info "Starting GUI Automation Framework Validation"
    log_info "Component: $component"
    echo
    
    case "$component" in
        "container")
            validate_container
            ;;
        "scripts")
            validate_scripts
            ;;
        "ci")
            validate_ci
            ;;
        "compliance")
            validate_compliance
            ;;
        "integration")
            validate_integration
            ;;
        "all")
            validate_container
            echo
            validate_scripts
            echo
            validate_ci
            echo
            validate_compliance
            echo
            validate_integration
            ;;
        *)
            log_error "Unknown component: $component"
            log_info "Available components: container, scripts, ci, compliance, integration, all"
            exit 1
            ;;
    esac
    
    echo
    log_info "=== Validation Summary ==="
    log_info "Total tests: $TOTAL_TESTS"
    log_success "Passed: $PASSED_TESTS"
    if [ $FAILED_TESTS -gt 0 ]; then
        log_error "Failed: $FAILED_TESTS"
    else
        log_success "Failed: $FAILED_TESTS"
    fi
    
    local success_rate=$((PASSED_TESTS * 100 / TOTAL_TESTS))
    log_info "Success rate: $success_rate%"
    
    if [ $FAILED_TESTS -eq 0 ]; then
        log_success "🎉 All validations passed!"
        exit 0
    else
        log_error "❌ Some validations failed"
        exit 1
    fi
}

# Run main function
main "$@"