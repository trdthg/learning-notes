from pymouse import *     # 模拟鼠标所使用的包
from pykeyboard import *   # 模拟键盘所使用的包
import time   # 连续进行两个动作可能太快而效果不明显，因此加入暂停时间

m = PyMouse()   # 鼠标的实例m
k = PyKeyboard()   # 键盘的实例k

x_dim, y_dim = m.screen_size()     # 获取屏幕尺寸（一般为电脑屏幕的分辨率，如1920*1080）
print(x_dim, " ", y_dim)
# 估计需要点击的位置坐标（不知道有没有定位代码，我没找到，我是自己估计的。例如，我的电脑屏幕为(1920，1080)，我想要单击的地方估计坐标为(10，500)）
m.move(int(x_dim/2), int(y_dim/2))   # 将鼠标移动到位（此步可忽略，直接单击也可）
time.sleep(0.5)   # 暂停0.5s，方便观察移动结果
m.click(int(x_dim/2), int(y_dim/2), 1, 1)   # 表示在(10, 500)的地方，单击左键

k.type_string('hello world')   # 模拟键盘输入字符串
k.press_key("Enter") # –模拟键盘按H键 
k.release_key("Enter") # –模拟键盘松开H键 hello world