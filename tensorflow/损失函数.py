import tensorflow as tf
import numpy as np

seed = 23455
rdm = np.random.RandomState(seed = seed)
x = rdm.rand(32, 2)
print(type(x),x.dtype)  #默认为float64
y_ = [[x1 + x2 + rdm.rand() / 10.0 - 0.05] for (x1, x2) in x] # 自制数据集,随机生成[-0.05, 0.05]的噪声
y_ = np.array(y_)
print(type(y_),y_.dtype)
x = tf.cast(x, dtype = tf.float32)
y_ = tf.cast(y_, dtype = tf.float32)

w1 = tf.Variable(tf.random.normal([2,1], stddev=1, seed=1))
epoch = 15000
lr = 0.002
PROFIT = 99
COST = 1

for epoch in range(epoch):
    with tf.GradientTape() as tape:
        y = tf.matmul(x, w1)


        #1 均方误差
        loss_mse = tf.reduce_mean(tf.square(y_ - y))


        #2 自定义损失函数  (y-y_)*COST, (y_-y)*PROFIT)
        loss = tf.reduce_mean(tf.where(tf.greater(y, y_), (y-y_)*COST, (y_-y)*PROFIT))

        #3 交叉熵损失函数  H(y_, y) = -Σy*ln y   H越小,越接近
        # softmax与交叉熵结合
        y_pro = tf.nn.softmax(y)
        loss_ce1 = tf.losses.categorical_crossentropy(y_,y_pro)
        loss_ce2 = tf.nn.softmax_cross_entropy_with_logits(y_, y)
    # print(loss_ce1,'+ohmygod+',loss_ce2)
        

    grads = tape.gradient(loss_ce1, w1)
    w1.assign_sub(lr * grads)
    if epoch%500==0:
        print(epoch, loss_ce1, loss_ce2)
        # pass
    # break