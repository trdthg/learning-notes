import matplotlib.pyplot as plt
import numpy as np

n = 1024
X = np.random.normal(0,1,n)
Y = np.random.normal(0,1,n)
T = np.arctan2(Y,X) # for color value

plt.scatter(X,Y, s=75, c=T, alpha=0.5, )
# plt.scatter(np.arange(5), np.arange(5) )
plt.xlim((-2,2))
plt.ylim((-2,2))


# 去掉坐标轴
plt.xticks(())
plt.yticks(())
# 去掉边框
ax = plt.gca()
for spin in ax.spines:
    ax.spines[spin].set_color('none')
# ax.spines['top'].set_color('none')

plt.show()