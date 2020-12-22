import matplotlib.pyplot as plt
import numpy as np

n=12
X = np.arange(n)
Y1 = (1-X/float(n)) * np.random.uniform(0.5,1.0,n)
Y2 = (1-X/float(n)) * np.random.uniform(0.5,1.0,n)
# 1-n/float(n) 用来生成随机因子
plt.bar(X,+Y1, facecolor='#444')
plt.bar(X,-Y2, edgecolor='#894')

for x,y1,y2 in zip(X, Y1, Y2):
    # zip 一次输出两个值
    plt.text(x, y1, f'{y1:.2}', ha='center', va='bottom')
    plt.text(x, -y2-0.07, f'{-y2:.2}', ha='center', va='bottom')

plt.show()