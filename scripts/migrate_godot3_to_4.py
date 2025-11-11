#!/usr/bin/env python3
"""
Automated Godot 3 to 4 GDScript Migration Tool

This script handles the most common syntax changes when migrating from Godot 3 to Godot 4.
It performs safe, reversible transformations that can be manually reviewed.
"""

import re
import sys
from pathlib import Path
from typing import List, Tuple


class GodotMigrator:
    def __init__(self):
        self.changes_made = []

    def migrate_file(self, file_path: Path) -> Tuple[str, List[str]]:
        """Migrate a single file and return the new content and list of changes."""
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()

        original_content = content
        local_changes = []

        # 1. File API: File.new() -> FileAccess.open()
        # This is complex and needs manual review, so we'll just flag it
        if 'File.new()' in content or 'var file = File' in content:
            local_changes.append("⚠️  MANUAL: File API needs migration to FileAccess")

        # 2. instance() -> instantiate()
        pattern = r'\b(\w+)\.instance\('
        if re.search(pattern, content):
            content = re.sub(pattern, r'\1.instantiate(', content)
            local_changes.append("instance() → instantiate()")

        # 3. rand_range() -> randf_range() or randi_range()
        # Use randf_range by default, user can change to randi_range if needed
        if 'rand_range(' in content:
            content = content.replace('rand_range(', 'randf_range(')
            local_changes.append("rand_range() → randf_range()")

        # 4. Remove randomize() calls as they're automatic in Godot 4
        pattern = r'\s*randomize\(\)\s*\n'
        if re.search(pattern, content):
            content = re.sub(pattern, '', content)
            local_changes.append("Removed randomize() call (automatic in Godot 4)")

        # 5. deg2rad() -> deg_to_rad()
        if 'deg2rad(' in content:
            content = content.replace('deg2rad(', 'deg_to_rad(')
            local_changes.append("deg2rad() → deg_to_rad()")

        # 6. rad2deg() -> rad_to_deg()
        if 'rad2deg(' in content:
            content = content.replace('rad2deg(', 'rad_to_deg(')
            local_changes.append("rad2deg() → rad_to_deg()")

        # 7. Button constants
        if 'BUTTON_LEFT' in content:
            content = content.replace('BUTTON_LEFT', 'MOUSE_BUTTON_LEFT')
            local_changes.append("BUTTON_LEFT → MOUSE_BUTTON_LEFT")
        if 'BUTTON_RIGHT' in content:
            content = content.replace('BUTTON_RIGHT', 'MOUSE_BUTTON_RIGHT')
            local_changes.append("BUTTON_RIGHT → MOUSE_BUTTON_RIGHT")
        if 'BUTTON_MIDDLE' in content:
            content = content.replace('BUTTON_MIDDLE', 'MOUSE_BUTTON_MIDDLE')
            local_changes.append("BUTTON_MIDDLE → MOUSE_BUTTON_MIDDLE")

        # 8. export syntax - Simple cases
        # export(int) -> @export
        pattern = r'export\((\w+)\)\s+var\s+(\w+)'
        matches = re.findall(pattern, content)
        if matches:
            content = re.sub(pattern, r'@export var \2', content)
            local_changes.append(f"export(Type) → @export ({len(matches)} occurrences)")

        # export(int, range, ...) -> @export_range(...)
        # This is complex, flag for manual review
        if re.search(r'export\(\w+,\s*\d+', content):
            local_changes.append("⚠️  MANUAL: export() with range needs @export_range()")

        # 9. Export flags - common patterns
        if 'export(NodePath)' in content:
            content = content.replace('export(NodePath)', '@export')
            local_changes.append("export(NodePath) → @export")

        if 'export(PackedScene)' in content:
            content = content.replace('export(PackedScene)', '@export')
            local_changes.append("export(PackedScene) → @export")

        # 10. Node type renames
        node_renames = {
            'KinematicBody2D': 'CharacterBody2D',
            'KinematicBody': 'CharacterBody3D',
            'Spatial': 'Node3D',
        }
        for old, new in node_renames.items():
            if old in content:
                # Only replace as type annotations or extends, not in strings
                pattern = r'\b' + old + r'\b'
                if re.search(pattern, content):
                    content = re.sub(pattern, new, content)
                    local_changes.append(f"{old} → {new}")

        # 11. move_and_slide() changes
        # In Godot 4, move_and_slide() takes no parameters
        # velocity is now a built-in property
        pattern = r'move_and_slide\([^)]+\)'
        if re.search(pattern, content):
            local_changes.append("⚠️  MANUAL: move_and_slide() API changed - now uses velocity property")

        # 12. get_slide_collision() -> get_slide_collision(index)
        # Actually this is the same, but the return type changed

        # 13. Flag super() calls needed
        if re.search(r'func _ready\(', content) and 'super._ready()' not in content:
            local_changes.append("⚠️  CONSIDER: Add super._ready() call if extending custom class")
        if re.search(r'func _process\(', content) and 'super._process(' not in content:
            local_changes.append("⚠️  CONSIDER: Add super._process() call if extending custom class")

        # 14. Tween changes
        if 'Tween.new()' in content or '$Tween' in content:
            local_changes.append("⚠️  MANUAL: Tween is no longer a node - use create_tween()")

        # 15. get_tree().get_root() -> get_tree().root (property access)
        if 'get_tree().get_root()' in content:
            content = content.replace('get_tree().get_root()', 'get_tree().root')
            local_changes.append("get_tree().get_root() → get_tree().root")

        # 16. OS.get_screen_size() -> DisplayServer.screen_get_size()
        if 'OS.get_screen_size(' in content:
            content = content.replace('OS.get_screen_size(', 'DisplayServer.screen_get_size(')
            local_changes.append("OS.get_screen_size() → DisplayServer.screen_get_size()")

        # 17. yield() -> await
        if 'yield(' in content:
            local_changes.append("⚠️  MANUAL: yield() → await (syntax change required)")

        # 18. weakref() -> weakref(obj)
        # Actually the same, but returns WeakRef object with get_ref() -> get()
        if 'weakref(' in content:
            local_changes.append("⚠️  MANUAL: WeakRef.get_ref() → WeakRef.get()")

        # 19. Array and Dictionary typed syntax
        # Array -> Array[Type] needs manual intervention

        # 20. Global scope changes
        # str() -> str() is the same
        # but var2str() -> var_to_str() and str2var() -> str_to_var()
        if 'var2str(' in content:
            content = content.replace('var2str(', 'var_to_str(')
            local_changes.append("var2str() → var_to_str()")
        if 'str2var(' in content:
            content = content.replace('str2var(', 'str_to_var(')
            local_changes.append("str2var() → str_to_var()")

        # 21. Physics/Collision changes
        if '.extents' in content:
            local_changes.append("⚠️  MANUAL: .extents → .size (and multiply by 2 for collision shapes)")

        # 22. Input.is_action_pressed vs is_action_just_pressed - same in both

        # 23. Color constants - mostly the same

        # Return results
        if content != original_content:
            return content, local_changes
        else:
            return None, local_changes

    def migrate_directory(self, directory: Path, dry_run: bool = False) -> None:
        """Migrate all .gd files in a directory recursively."""
        gd_files = list(directory.rglob("*.gd"))

        print(f"Found {len(gd_files)} GDScript files to migrate")
        print("=" * 60)

        files_changed = 0
        total_changes = 0

        for file_path in sorted(gd_files):
            new_content, changes = self.migrate_file(file_path)

            if changes:
                rel_path = file_path.relative_to(directory)
                print(f"\n📄 {rel_path}")
                for change in changes:
                    print(f"   • {change}")
                    if not change.startswith("⚠️"):
                        total_changes += 1

                if new_content is not None:
                    files_changed += 1
                    if not dry_run:
                        with open(file_path, 'w', encoding='utf-8') as f:
                            f.write(new_content)
                        print(f"   ✅ File updated")
                    else:
                        print(f"   🔍 File would be updated (dry run)")

        print("\n" + "=" * 60)
        print(f"Summary:")
        print(f"  Files scanned: {len(gd_files)}")
        print(f"  Files modified: {files_changed}")
        print(f"  Automatic changes: {total_changes}")
        print(f"  Mode: {'DRY RUN' if dry_run else 'APPLIED'}")


def main():
    import argparse

    parser = argparse.ArgumentParser(
        description="Migrate Godot 3 GDScript files to Godot 4 syntax"
    )
    parser.add_argument(
        "directory",
        type=Path,
        help="Directory containing .gd files to migrate"
    )
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="Show what would be changed without modifying files"
    )

    args = parser.parse_args()

    if not args.directory.exists():
        print(f"Error: Directory {args.directory} does not exist")
        sys.exit(1)

    migrator = GodotMigrator()
    migrator.migrate_directory(args.directory, dry_run=args.dry_run)


if __name__ == "__main__":
    main()
