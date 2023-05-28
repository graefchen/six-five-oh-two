#!/usr/bin/env nu

# Just a small nu script to make the log file for testing and learning...
# needs updating to maybe really be a ".log" - file

def main [] {
	cargo run | save "output.txt" --force
}