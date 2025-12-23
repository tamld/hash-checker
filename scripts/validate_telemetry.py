#!/usr/bin/env python3
import argparse
import sys
import os

def parse_kv(line):
    parts = line.strip().split(',')
    if len(parts) < 2:
        return None
    # Timestamp is first
    data = {'timestamp': parts[0]}
    for p in parts[1:]:
        if '=' in p:
            k, v = p.split('=', 1)
            data[k] = v
    return data

def validate(log_path, operation, checks):
    if not os.path.exists(log_path):
        print(f"Error: Log file {log_path} not found.")
        sys.exit(1)

    with open(log_path, 'r') as f:
        lines = f.readlines()

    if not lines:
        print("Error: Log file is empty.")
        sys.exit(1)

    # Find last entry matching operation (case insensitive for safety, though code uses title case)
    last_entry = None
    target_op = operation.lower()

    for line in reversed(lines):
        entry = parse_kv(line)
        if entry and entry.get('operation', '').lower() == target_op:
            last_entry = entry
            break

    if not last_entry:
        print(f"Error: No entry found for operation {operation}.")
        sys.exit(1)

    print(f"Found entry: {last_entry}")

    failed = False
    for k, v in checks.items():
        if k not in last_entry:
            print(f"Failure: Key {k} missing in log entry.")
            failed = True
        elif last_entry[k] != str(v):
            print(f"Failure: Expected {k}={v}, got {last_entry[k]}")
            failed = True

    if failed:
        sys.exit(1)
    print("Validation passed.")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Validate Hash Checker GUI telemetry logs.")
    parser.add_argument("log_file", help="Path to telemetry.log")
    parser.add_argument("operation", help="Operation to find (Scan/Verify)")
    parser.add_argument("--expect", action="append", help="Expected key=value pair (e.g. recorded=10)")
    args = parser.parse_args()

    checks = {}
    if args.expect:
        for e in args.expect:
            if '=' in e:
                k, v = e.split('=', 1)
                checks[k] = v
            else:
                print(f"Warning: Invalid expect format '{e}', ignoring.")

    validate(args.log_file, args.operation, checks)
