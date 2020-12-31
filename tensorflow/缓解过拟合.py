import csv
import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
import tensorflow as tf

# ————1.获取数据————
df = pd.read_csv('tensorflow/dot.csv')
x_data = np.array(df[['x1', 'x2']])
y_data = np.array(df['y_c'])

# reshape函数能修改数组形状, 且不占用新的内存空间
# 参数-1表示根据另一个参数推测, 只有-1就是一行
x_train = np.vstack(x_data).reshape(-1,2)
y_train = np.vstack(y_data).reshape(-1,1)  

Y_c = [['red' if y else 'blue'] for y in y_train]
# print(x_train.dtype,y_train.dtype)  float64 int64
x_train = tf.cast(x_train, tf.float32)
y_train = tf.cast(y_train, tf.float32)

for sssss in range(10,11):
    train_db = tf.data.Dataset.from_tensor_slices((x_train, y_train) ).batch(sssss*3)

    # ————2.设置神经元————
    # 2为输入层  100为隐藏层, 初始值为随机正态分布
    w1 = tf.Variable(tf.random.normal([2,100]), dtype=tf.float32)
    b1 = tf.Variable(tf.constant(0.01, shape=[100]))
    # !!!!!!!!!!-------有几个输出shape就填几--------!!!!!!!!!!!!!!!!!

    w2 = tf.Variable(tf.random.normal([100,1]), dtype=tf.float32)
    b2 = tf.Variable(tf.constant(0.01, shape=[1]))

    # ————3.训练部分————
    lr_base = 0.01
    lr_decay = 0.99
    lr_step = 100
    epoch = 400
    for epoch in range(epoch):
        lr = lr_base * lr_decay ** (epoch / lr_step)

        for step, (x_train, y_train) in enumerate(train_db):
            with tf.GradientTape() as tape:
                h1 = tf.matmul(x_train, w1) + b1
                # 进入隐藏层前使用激活函数
                h1 = tf.nn.relu(h1)
                y = tf.matmul(h1, w2) + b2
                # 1. 采用均方误差
                loss_mse = tf.reduce_mean(tf.square(y_train-y))
                # 2.加入L2正则化
                loss_regularization = []
                # tf.nn.l2_loss(w) = sum(w**2) / 2

                loss_regularization.append(tf.nn.l2_loss(w1))
                loss_regularization.append(tf.nn.l2_loss(w2))
                loss_regularization = tf.reduce_sum(loss_regularization)
                loss = loss_mse
                loss = loss_mse + 0.03 * loss_regularization
            variables = [w1, b1, w2, b2]
            grads = tape.gradient(loss, variables)
            w1.assign_sub(lr * grads[0])
            b1.assign_sub(lr * grads[1])
            w2.assign_sub(lr * grads[2])
            b2.assign_sub(lr * grads[3])
        # if epoch%100==0:
            # print(epoch,float(loss))

    # ————4.预测部分————
    # 将网格坐标点作为测试集喂入网络，进行预测
    xx, yy = np.mgrid[-3:3:.1, -3:3:.1]
    print(xx.shape)
    grid = np.c_[xx.ravel(), yy.ravel()]
    grid = tf.cast(grid, tf.float32)
    probs = []
    for x_test in grid:
        h1 = tf.matmul([x_test], w1) + b1
        h1 = tf.nn.relu(h1)
        y = tf.matmul(h1, w2) + b2
        probs.append(y)
    # X[:, m:n]，即取所有数据的第m到n-1列数据，左闭右开
    x1 = x_data[:,0]
    x2 = x_data[:,1]
    print(np.array(probs).shape)
    probs = np.array(probs).reshape(xx.shape)
    plt.figure()
    plt.scatter(x1,x2,color=np.squeeze(Y_c))
    plt.contour(xx, yy, probs, levels=[0.5])
plt.show()
