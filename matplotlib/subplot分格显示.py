import matplotlib.pyplot as plt
import matplotlib.gridspec as gridspec

# method 1: subplot2grid
plt.figure()
ax1 = plt.subplot2grid((3,3), (0,0), colspan=3, rowspan=1)
ax1.plot([1,2],[2,4])
ax1.set_title('aaa')

ax2 = plt.subplot2grid((3,3), (2,0), colspan=1, rowspan=1)
ax2.plot([0,5],[2,2])
ax2.set_title('bbb')

ax3 = plt.subplot2grid((3,3), (1,2), colspan=1, rowspan=2)
ax3.set_title('ccc')

# method 2: gridspec
# 索引切片表示
plt.figure()
gs = gridspec.GridSpec(4,4)
ax1 = plt.subplot(gs[0,:])
# ax2 = plt.subplot(gs[1,:3])
ax3 = plt.subplot(gs[1:,3])
ax4 = plt.subplot(gs[-1,0])
# ax5 = plt.subplot(gs[-1,-2])

# method 3: easy to define structure
figureobject, ((ax11,ax22),(ax21,ax22)) = plt.subplots(2,2, sharex=True, sharey=False)
ax11.scatter([1,2],[0,4])


plt.show()