import platform
from ctypes import *

libc = cdll.LoadLibrary('msvcrt.dll')

def to_cstring(pystr):
    return create_string_buffer(pystr.encode("utf-8"))
    
libc.printf(b'Hello ctypes!\n')

libc.printf(to_cstring('Hello ctypes!\n'))

lib = CDLL('./pkg/lms.dll')
a = lib.add(1, 2)
print(a)