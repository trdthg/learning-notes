import matplotlib.pyplot as plt
import numpy as np
from mpl_toolkits.mplot3d import Axes3D
from mpl_toolkits import mplot3d

fig = plt.figure()
ax = Axes3D(fig)
X = np.arange(-4,4,0.25)
Y = np.arange(-4,4,0.25)
X, Y = np.meshgrid(X,Y)
R = np.sqrt(X**2 + Y**2)
Z = np.cos(R)
ax.plot_surface(X, Y, Z, rstride=1, cstride=1, cmap=plt.get_cmap('rainbow'))

# ax = plt.axes(projection='3d')
# X = np.arange(-4,4,0.25)
# Y = np.arange(-4,4,0.25)
# X, Y = np.meshgrid(X,Y)
# R = np.sqrt(X**2 + Y**2)
# Z = np.cos(R)
# ax.plot3D(X, Y, Z, 'gray')

ax.contourf(X, Y, Z, zdir='z', offset=-4, edgecolor='black', cmap='rainbow')
ax.set_zlim(-4,4)   # offset 为z轴位置
plt.show()