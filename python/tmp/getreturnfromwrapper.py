from functools import wraps
def test(func):
    @wraps(func)
    def inner_func():
        print("inner")
        return func("outer")
    return inner_func

@test
def test_func(obj):
    print(obj)

a = test_func()


