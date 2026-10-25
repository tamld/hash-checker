#!/usr/bin/env python3
import argparse
import os
import sys


def parse_kv(line):
    parts = line.strip().split(",")
    if len(parts) < 2:
        return None
    data = {"timestamp": parts[0]}
    for part in parts[1:]:
        if "=" in part:
            k, v = part.split("=", 1)
            data[k] = v
    return data


def validate(log_path, operation, checks):
    if not os.path.exists(log_path):
        print(f"Error: Log file {log_path} not found.")
        sys.exit(1)

    with open(log_path, "r", encoding="utf-8") as handle:
        lines = handle.readlines()

    if not lines:
        print("Error: Log file is empty.")
        sys.exit(1)

    last_entry = None
    target_op = operation.lower()

    for line in reversed(lines):
        entry = parse_kv(line)
        if entry and entry.get("operation", "").lower() == target_op:
            last_entry = entry
            break

    if not last_entry:
        print(f"Error: No entry found for operation {operation}.")
        sys.exit(1)

    print(f"Found entry: {last_entry}")

    failed = False
    for key, val in checks.items():
        if key not in last_entry:
            print(f"Failure: Key {key} missing in log entry.")
            failed = True
        elif last_entry[key] != str(val):
            print(f"Failure: Expected {key}={val}, got {last_entry[key]}")
            failed = True

    if failed:
        sys.exit(1)
    print("Validation passed.")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Validate Hash Checker GUI telemetry logs.")
    parser.add_argument("log_file", help="Path to telemetry.log")
    parser.add_argument("operation", help="Operation to find (Scan/Verify)")
    parser.add_argument(
        "--expect", action="append", help="Expected key=value pair (e.g. recorded=10)"
    )
    args = parser.parse_args()

    checks = {}
    if args.expect:
        for item in args.expect:
            if "=" in item:
                k, v = item.split("=", 1)
                checks[k] = v
            else:
                print(f"Warning: Invalid expect format '{item}', ignoring.")

    validate(args.log_file, args.operation, checks)
