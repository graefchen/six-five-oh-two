#!/usr/bin/env nu

# a file to recompile the asm code from the zip file in this repo:
# https://github.com/Klaus2m5/6502_65C02_functional_tests/tree/master
# the zip file should be unzipped in the bin directory

def main [] {
    .\bin\as65_142\as65.exe -l -m -w -h0 `bin\6502_functional_test.a65`
}