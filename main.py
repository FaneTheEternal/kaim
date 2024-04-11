import kaim

profiler = kaim.Profiler()


def foo(s):
    print(s)


with profiler:
    foo(profiler)
foo('qwerty')
