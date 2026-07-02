#!/usr/bin/env python3

import sys


def main():
    # Grimoire passes the injected {{target}} argument as sys.argv[1]
    target = sys.argv[1] if len(sys.argv) > 1 else "Unknown"
    print(f"🐍 Python Sigil: Casting a spell on {target}!")


if __name__ == "__main__":
    main()
