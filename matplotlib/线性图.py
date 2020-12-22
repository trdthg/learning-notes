import matplotlib.pyplot as plt
import numpy as np

# 第一个图象
x=np.linspace(-3,3,50)
y1 = 2*x+1
y2 = x*x
plt.figure()
    # 画图并且设置图例
l1, = plt.plot(x,y1, label='up')
l2, = plt.plot(x,y2, label='down')
plt.legend(handles=[l1,l2], labels=['y=2*x+1','y=x*x'],loc='best')
    # 坐标轴位置调整
ax = plt.gca() ## get current axis
ax.spines['right'].set_color('none')
ax.spines['top'].set_color('none')
ax.yaxis.set_ticks_position('left')
ax.xaxis.set_ticks_position('bottom')
ax.spines['bottom'].set_position(('data',0))
ax.spines['left'].set_position(('data',0))
    # 任意两点间连线  
x0 = 1
y0 = 2*x0+ 1
plt.scatter(x0,y0, s=10, color='b')
plt.plot([x0,x0],[y0,0], 'k--', lw=1.5)
    # method 1
plt.annotate(f'当x=1时y的取值{y0}', xy=(x0,y0), xycoords='data', xytext=(+30,-30),
textcoords='offset points',fontsize=16,fontproperties="FangSong",arrowprops=dict(arrowstyle='->',connectionstyle='arc3, rad=2'))
    # method2
plt.text(-3.7, 3,  r'$\mu\ \sigma_i\ \alpha_t\ $',fontdict={'size':16})  # 下划线后的是脚标
    # x轴ticks样式设置
for label in ax.get_xticklabels() + ax.get_yticklabels():
    label.set_fontsize(6)
    label.set_bbox(dict(facecolor='lightblue', edgecolor='red', alpha=0.7))
# 第二个图像
plt.figure(num=3,figsize=(10,5))
plt.plot(x,y2)
plt.plot(x,y1,color='red',linewidth='5.0', linestyle='--')

plt.xlim((-1,2))
plt.ylim((-2,3))
plt.xlabel(r"水平坐标轴",fontproperties="FangSong")
plt.ylabel("Oh my God")

new_ticks = np.linspace(-1,2,20)
plt.xticks(new_ticks)


plt.yticks([-2, -1.8, -1, 0, 1.22, 3],[r'$first$','$second$','$thord$', '0',r'$forth\ \alpha\ $','$fif\ th$'])

plt.show()
# 有用，即设置(set)x的刻度(xaxis_ticks)的位置(position)为bottom，
# 即x轴的刻度显示在bottom边上,将bottom换成top你就彻底明白了