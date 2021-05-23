from matplotlib import pyplot as plt
import numpy as np
from matplotlib import animation 

fig, ax = plt.subplots()
x = np.arange(0, 2*np.pi, 0.01)
line, = ax.plot(x, np.sin(x))

def animate(i):
    line.set_ydata(np.sin(x + i/10))
    return line,
def init():
    line.set_ydata(np.sin(x))
    return line,
ani = animation.FuncAnimation(fig=fig, func=animate, frames=10, init_func=init, interval=1, blit=False)
plt.show()