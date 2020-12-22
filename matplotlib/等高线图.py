import matplotlib.pyplot as plt
import numpy as np

def height(x,y):
    return (1-x/2 + x**5 + y**3) * np.exp(-x**2 - -y**2)

n = 256
x = np.linspace(-3, 3, n)
y = np.linspace(-3, 3, n)

X, Y = np.meshgrid(x,y)








plt.show()
