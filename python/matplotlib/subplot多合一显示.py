import matplotlib.pyplot as plt
plt.figure()
# 2行中第1行的第1个
plt.subplot(2,1,1)
plt.plot([0,1],[0,1])

# 2行中第二行的第1个(4=1+3)
plt.subplot(2,3,4)
plt.plot([0,1],[0,2])

plt.subplot(235)
plt.plot([0,1],[0,3])

plt.subplot(236)
plt.plot([0,1],[0,4])

plt.show()
