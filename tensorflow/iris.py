import time
# -*- coding:UTF-8 -*-
import tensorflow as tf
from sklearn import datasets
import matplotlib.pyplot as plt
import numpy as np
from sklearn.model_selection import train_test_split
first_time = time.time()

print('---begin---')
print('导入数据集')
x_data = datasets.load_iris().data
y_data = datasets.load_iris().target

# print('数据据乱序')
# np.random.seed(116)
# np.random.shuffle(x_data)
# np.random.seed(116)
# np.random.shuffle(y_data)
# tf.random.set_seed(116)
# print('分割为训练集和测试集')
# x_train = x_data[:-30]
# y_train = y_data[:-30]
# x_test = x_data[-30:]
# y_test = y_data[-30:]

x_train, x_test, y_train, y_test = train_test_split(x_data, y_data, test_size=0.5, random_state=432)

# 保证输入的特征和标签类型相同, 防止矩阵相乘报错
x_train = tf.cast(x_train, tf.float32)
x_test = tf.cast(x_test, tf.float32)

# 输入特征与标签一一对应
train_db = tf.data.Dataset.from_tensor_slices((x_train, y_train)).batch(1)
test_db = tf.data.Dataset.from_tensor_slices((x_test, y_test)).batch(1)

# 生成神经网络的参数
w1 = tf.Variable(tf.random.truncated_normal([4,3], stddev=0.1, seed=1))
b1 = tf.Variable(tf.random.truncated_normal([3], stddev=0.1, seed=1))

# 指数衰减学习率
train_loss_results = []
test_acc = []  

lr_base = 0.1
lr_decay = 0.99
lr_step = 1
epoch = 100
loss_all = 0

# print('---开始训练(摸鱼)---')
for epoch in range(epoch):
    lr = lr_base * lr_decay ** (epoch / lr_step)
    for step, (x_train, y_train) in enumerate(train_db):
        with tf.GradientTape() as tape:
            y = tf.matmul(x_train, w1) + b1               # 神经网络乘加运算
            y = tf.nn.softmax(y)                          # 使输出y符合概率分布（此操作后与独热码同量级，可相减求loss）
            y_ = tf.one_hot(y_train, depth=3)             # 将标签值转换为独热码格式，方便计算loss和accuracy
            # print(step,y)
            loss = tf.reduce_mean(tf.square(y_ - y))      # 采用均方误差损失函数mse = mean(sum(y-out)^2)
            # print(y-y_, loss)
            loss_all += loss.numpy()                      # 将每个step计算出的loss累加，为后续求loss平均值提供数据，这样计算的loss更准确
        #计算loss对各个参数的梯度
        grads = tape.gradient(loss, [w1, b1])
        # 实现梯度更新 w1 = w1 - lr * w1_grad    b = b - lr * b_grad
        w1.assign_sub(lr * grads[0])
        b1.assign_sub(lr * grads[1])
    # print('---一个epoch已经 over---')

    # print(f'{epoch}, {loss_all/4}   Σ(っ°Д °;)っ🐟')

    train_loss_results.append(loss_all/4)
    # print('---开始测试---')
    total_correct, total_number = 0, 0
    for x_test, y_test in test_db:
        # 用更新后的参数进行预测
        # print(y_test)
        y = tf.matmul(x_test, w1) + b1
        # print(y)
        y = tf.nn.softmax(y)
        # print(y)
        pred = tf.argmax(y, axis=1)  # 返回y中最大值的索引, 及预测的分类
        # print(pred)
        # 将pred转换为y_test的数据类型
        pred = tf.cast(pred, dtype=y_test.dtype)
        # 若分类正确,correct=1, 否则为0, 将bool转换为int
        # print(pred, y_test)
        correct = tf.cast(tf.equal(pred, y_test), dtype=tf.int32)
        # print(correct)
        correct = tf.reduce_sum(correct)
        total_correct += int(correct)
        total_number += x_test.shape[0]
    acc = total_correct/total_number
    test_acc.append(acc)
    print(f'test_acc: {acc}  {lr} {loss_all}')
    loss_all = 0
second_time = time.time()
print(first_time-second_time)

plt.figure()
plt.title('Loss Function Curve')
plt.xlabel('Epoch')
plt.ylabel('Loss')
plt.plot(train_loss_results, label='$Loss$')
plt.legend()

plt.figure()
plt.title('Acc Curve')
plt.xlabel('Epoch')
plt.ylabel('Acc')
plt.plot(test_acc, label='$Accuracy$')
plt.legend()
plt.show()