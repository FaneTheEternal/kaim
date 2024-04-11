import faulthandler
from pathlib import Path

import kaim

faulthandler.enable()

profiler = kaim.Profiler()


def foo(n):
    if n <= 1:
        return 1
    return n + foo(n - 1)


def bar(n):
    for i in range(n):
        yield i


def main():
    result = []
    for i in bar(3):
        result.append(i)
    for i in bar(5):
        result.append(i)
    print(result)


with profiler:
    main()

print('done\n\n')

entries = sorted(profiler.get_entries(), key=lambda e: e.time[0])

offset = 0
stack = []
for entry in entries:
    ts = entry.time
    while stack and stack[-1][1] <= ts[1]:
        stack.pop()
    stack.append(ts)

    print(
        ' ' * (len(stack) - 1),
        f'{entry.info} on {entry.called}',
    )

with open(Path.home().joinpath('Downloads', 'prof.ron'), 'w') as f:
    f.write(profiler.dump())
