import numpy as np
import tensorflow as tf
from tensorflow.keras.layers import Dense, SimpleRNN, Dropout, LSTM, GRU
import matplotlib.pyplot as plt
import os
import pandas as pd
from sklearn.preprocessing import MinMaxScaler
from sklearn.metrics import mean_squared_error, mean_absolute_error
import math
import datetime


summary_writer = tf.summary.create_file_writer('./tensorboard')     # 参数为记录文件所保存的目录


# 共2499条数据, 第一条是表头
maotai = pd.read_csv('./tensorflow/SH600519.csv')
print(maotai)
all_set = maotai.iloc[:,2:3].values
train_set = maotai.iloc[0:1213,2:3].values

sc = MinMaxScaler(feature_range=(0,1))
train_set_scaled = sc.fit_transform(train_set)
x_train = []
y_train = []

for i in range(60, len(train_set_scaled)):
    x_train.append(train_set_scaled[i-60:i, 0])
    y_train.append(train_set_scaled[i, 0])
np.random.seed(7)
np.random.shuffle(x_train)
np.random.seed(7)
np.random.shuffle(y_train)
tf.random.set_seed(7)

x_train, y_train = np.array(x_train), np.array(y_train)
x_train = np.reshape(x_train, (x_train.shape[0], 60 , 1))



model = tf.keras.Sequential([
    # SimpleRNN(80, return_sequences=True),  # 两层都是RNN时,前一层要加上return_sequences=True
    GRU(80, return_sequences=True),
    Dropout(0.2),  # 随即扔掉一些神经元, 防止过拟合, 可以先设为0, 逐渐调大, 找到最优值
    SimpleRNN(100),
    Dropout(0.2),
    Dense(1),
])

model.compile(
    optimizer = tf.keras.optimizers.Adam(0.001),
    loss = tf.keras.losses.mean_squared_error,
)

checkpoint_save_path = './savedata/stockGRU.ckpt'
if os.path.exists(checkpoint_save_path + '.index'):
    print('-----------------load the model------------------')
    model.load_weights(checkpoint_save_path)

cp_callback = tf.keras.callbacks.ModelCheckpoint(
    filepath = checkpoint_save_path,
    save_weights_only = True,
    save_best_only = True,
    # monitor = 'var_loss', # 指定需要监测的值
)

log_dir="logs/fit/" + datetime.datetime.now().strftime("%Y%m%d-%H%M%S")
tb_callback = tf.keras.callbacks.TensorBoard(log_dir=log_dir, histogram_freq=1)

history = model.fit(
    x_train, y_train,
    batch_size=64, epochs=1,
    validation_split=0.2,
    validation_freq=1,
    callbacks = [cp_callback, tb_callback],
)

model.summary()      


test1 = maotai.iloc[:1213,2:3].values
test_set = maotai.iloc[1213-60:,2:3].values
test_set = sc.transform(test_set)
x_test = []
for i in range(60, len(test_set)):
    x_test.append(test_set[i-60:i, 0])
x_test = np.array(x_test)
x_test = np.reshape(x_test, (x_test.shape[0], 60, 1))
predicted_stock_price = model.predict(x_test)
predicted_stock_price = sc.inverse_transform(predicted_stock_price)
test1 = np.vstack((test1, predicted_stock_price))



i = 500
data60_ori = maotai.iloc[:1213,2:3].values
def pre(i, data60_ori):
    i -= 1
    if i > 0:

        data60 = data60_ori
        data60 = sc.transform(data60)
        readyfortest = []
        readyfortest.append(data60[len(data60)-60:len(data60),0])
        readyfortest = np.array(readyfortest)
        readyfortest = np.reshape(readyfortest, (readyfortest.shape[0], 60, 1))
        preprice = model.predict(readyfortest)
        preprice = sc.inverse_transform(preprice)
        data60_ori = np.vstack((data60_ori, preprice))
        pre(i, data60_ori)
    else: 
        print('------over------')
        print(data60_ori.shape)
        plt.plot(all_set, c='r', label='Real Price')
        plt.plot(data60_ori, c='b', label='Predict Price')
        plt.plot(test1, c='y', label='Predict Price')
        plt.legend()
        plt.show()
        
pre(i, data60_ori)

