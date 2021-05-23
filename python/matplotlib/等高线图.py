import matplotlib.pyplot as plt
import numpy as np
# the height function
def height(x,y):
    return (1-x/2 + x**5 + y**3) * np.exp(-x**2 -y**2)

n = 256
x = np.linspace(-3, 3, n)
y = np.linspace(-3, 3, n)
X, Y = np.meshgrid(x,y)
# use plt.contourf to filling contour 
# X, Y and value for (X, Y)point

# 涂色
plt.contourf(X, Y, height(X,Y), 8, alpha=0.75, cmap=plt.cm.hot)

# 画线 use plt.contour to add contour line
C = plt.contour(X, Y, height(X,Y), 8, color='black', linewidths=.5)

# plt.xticks(())
# plt.yticks(())
plt.clabel(C, inline=True, fontsize=10)

plt.show()
