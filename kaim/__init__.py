import functools
import sys

from kaim import kaim


class Profiler(kaim.Profiler):
    def __init__(self):
        self.__level = 0

    def start(self):
        if self.__level == 0:
            super().start()
        self.__level += 1

    def stop(self):
        if self.__level == 0:
            raise RuntimeError('Profiler already stopped')
        elif self.__level == 1:
            sys.setprofile(None)
            self.__level = 0
        else:
            self.__level -= 1

    def __enter__(self):
        self.start()

    def __exit__(self, exc_type, exc_val, exc_tb):
        self.stop()

    def __call__(self, fn):
        @functools.wraps(fn)
        def wrapped(*args, **kwargs):
            self.start()
            ret = fn(*args, **kwargs)
            self.stop()
            return ret

        return wrapped
