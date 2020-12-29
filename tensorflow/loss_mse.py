import tensorflow as tf
import numpy as np

seed = 23455

rdm = np.random.RandomState(seed = seed)
x = rdm.rand(32, 2)
print(x.dtype)  #默认为float64
y_ = [[x1 + x2 + rdm.rand() / 10.0 - 0.05] for (x1, x2) in x] # 自制数据集,随机生成[-0.05, 0.05]的噪声
x = tf.cast(x, dtype = tf.float32)

w1 = tf.Variable(tf.random.normal([2,1], stddev=1, seed=1))
epoch = 15000
lr = 0.002
PROFIT = 99
COST = 1

for epoch in range(epoch):
    with tf.GradientTape() as tape:
        y = tf.matmul(x, w1)
        # loss_mse = tf.reduce_mean(tf.square(y_ - y))
        # 自定义损失函数
        loss = tf.reduce_mean(tf.where(tf.greater(y, y_), (y-y_)*COST, (y_-y)*PROFIT))
    grads = tape.gradient(loss, w1)
    w1.assign_sub(lr * grads)
    if epoch%500==0:
        print(epoch, w1.numpy())