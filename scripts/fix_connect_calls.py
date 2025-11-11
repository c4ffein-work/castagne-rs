#!/usr/bin/env python3
"""
Fix Godot 3 connect() calls to Godot 4 Callable() syntax
"""

import re
import sys
from pathlib import Path


def fix_connect_calls(content: str) -> tuple[str, int]:
    """Fix connect() calls in the content."""
    # Pattern to match: .connect("signal_name", object, "method_name")
    # Also handles optional array parameter: .connect("signal_name", object, "method_name", [params])
    pattern = r'\.connect\("([^"]+)",\s*(\w+),\s*"([^"]+)"(\s*,\s*\[[^\]]*\])?\)'

    def replacer(match):
        signal_name = match.group(1)
        object_name = match.group(2)
        method_name = match.group(3)
        params = match.group(4) if match.group(4) else ""

        # Build the replacement
        if params:
            # With parameters
            return f'.connect("{signal_name}", Callable({object_name}, "{method_name}").bind({params.strip()[1:]}'
        else:
            # Without parameters
            return f'.connect("{signal_name}", Callable({object_name}, "{method_name}"))'

    new_content, count = re.subn(pattern, replacer, content)
    return new_content, count


def main():
    directory = Path("/home/user/castagne-rs/castagne_godot4")
    gd_files = list(directory.rglob("*.gd"))

    total_changes = 0
    files_modified = 0

    for file_path in sorted(gd_files):
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()

        new_content, count = fix_connect_calls(content)

        if count > 0:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(new_content)

            rel_path = file_path.relative_to(directory.parent)
            print(f"✅ {rel_path}: {count} connect() calls fixed")
            total_changes += count
            files_modified += 1

    print(f"\n📊 Summary: {total_changes} calls fixed in {files_modified} files")


if __name__ == "__main__":
    main()
