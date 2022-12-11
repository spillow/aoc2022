from dataclasses import dataclass
from typing import Optional
from typing import Callable

@dataclass
class FileEntry:
    name: str
    size: int

@dataclass
class DirEntry:
    parent: Optional['DirEntry']
    name: str
    dirs: list['DirEntry']
    files: list[FileEntry]

def get_dir(cwd: DirEntry, name: str) -> DirEntry:
    for d in cwd.dirs:
        if d.name == name:
            return d
    assert False

def compute_nested_sizes(root: DirEntry, visit: Callable[[int], None]) -> int:
    cnt = 0
    for d in root.dirs:
        cnt += compute_nested_sizes(d, visit)
    for f in root.files:
        cnt += f.size
    visit(cnt)
    return cnt

def build_filesystem(path: str) -> DirEntry:
    root = DirEntry(None, '/', [], [])
    cwd  = root
    with open('inputs/day07/input.txt') as f:
        for line in f:
            line = line.strip()
            match line.split(' '):
                case ['$', 'cd', '/']:
                    cwd = root
                case ['$', 'cd', '..']:
                    if cwd is not None and cwd.parent is not None:
                        cwd = cwd.parent
                    else:
                        assert False
                case ['$', 'cd', d]:
                    cwd = get_dir(cwd, d)
                case ['$', 'ls']:
                    continue
                case ['dir', d]:
                    cwd.dirs.append(DirEntry(cwd, d, [], []))
                case [size, fname] if size.isdigit():
                    cwd.files.append(FileEntry(fname, int(size)))
                case _:
                    assert False
    return root

def part_a(path: str):
    root = build_filesystem(path)
    total = 0
    def accum(v: int):
        nonlocal total
        if v <= 100_000:
            total += v
    compute_nested_sizes(root, accum)
    print(f"Part A: {total}")

def total_size(root: DirEntry) -> int:
    size = 0
    def accum(v: int):
        nonlocal size
        size = v
    compute_nested_sizes(root, accum)
    return size

def get_min_dir_size(root: DirEntry, min_size: int) -> int:
    size = 100_000_000
    def accum(v: int):
        nonlocal size
        if v < min_size:
            return
        size = min(size, v)
    compute_nested_sizes(root, accum)
    return size

def part_b(path: str):
    root = build_filesystem(path)
    size = total_size(root)
    to_delete = 30_000_000 - (70_000_000 - size)
    min_dir = get_min_dir_size(root, to_delete)
    print(f"Part B: {min_dir}")

def main():
    part_a('inputs/day07/input.txt')
    part_b('inputs/day07/input.txt')

if __name__ == '__main__':
    main()