import tensorflow as tf
from tensorflow.keras.preprocessing.image import ImageDataGenerator
# 相比数据增强增加的会用---隔开
#--------------------------------------------------------------
import os
 #打印保存的数据
import numpy as np 
np.set_printoptions(threshold=np.inf) # 超过多少显示省略号,  np.inf为无限大
import matplotlib.pyplot as plt
#--------------------------------------------------------------

# fashion_mnist = tf.keras.datasets.fashion_mnist
mnist = tf.keras.datasets.mnist
(x_train, y_train), (x_test, y_test) = mnist.load_data()
# (x_train, y_train), (x_test, y_test) = fashion_mnist.load_data()
x_train, x_test = x_train / 255.0, x_test / 255.0

x_train = x_train.reshape(x_train.shape[0], 28, 28, 1)
img_gen_train = ImageDataGenerator(
    rescale=1./1.,   # 把 0~255 归为  0~1 
    rotation_range=45,  # 随机45度旋转
    width_shift_range=.15, #宽度偏移
    height_shift_range=.15, #高度偏移
    horizontal_flip=True, #水平翻转
    zoom_range = 0.5, # 将图像随即缩放50%
)
img_gen_train.fit(x_train)

model = tf.keras.Sequential([
    tf.keras.layers.Flatten(),
    tf.keras.layers.Dense(128, activation='relu'),
    tf.keras.layers.Dense(10, activation='softmax'),
])

model.compile(  optimizer='adam',
                loss=tf.keras.losses.SparseCategoricalCrossentropy(from_logits=False),
                metrics=['sparse_categorical_accuracy'])

# 1.断点续训的实现
#--------------------------------------------------------------
checkpoint_save_path = "./tensorflow/savedata/mnist.ckpt"

if os.path.exists(checkpoint_save_path + '.index'):
    print('-----------------load the model---------------')
    model.load_weights(checkpoint_save_path)

cp_callback = tf.keras.callbacks.ModelCheckpoint(
    filepath = checkpoint_save_path,
    save_weights_only = True,
    save_best_only = True,
)

summary_writer = tf.summary.create_file_writer('./tensorboard')     # 参数为记录文件所保存的目录
history = model.fit(
    # x_train, y_train, batch_size=32, epochs=1,
    img_gen_train.flow(x_train, y_train, batch_size=32), epochs=200,
    validation_data = (x_test, y_test),
    validation_freq = 1,
    callbacks = [cp_callback],
)
# model.fit(img_gen_train.flow(x_train, y_train, batch_size=48), epochs=5, validation_data=(x_test,y_test), validation_freq=1)
#--------------------------------------------------------------



model.summary()
# 2.把数据写入文件的实现
#--------------------------------------------------------------
# print(model.trainable_variables)
with open('./tensorflow/savedata/fashion_mnist_data.txt', 'w') as file:
    for v in model.trainable_variables:
        file.write(str(v.name) + '\n')
        file.write(str(v.shape) + '\n')
        file.write(str(v.numpy()) + '\n')
#--------------------------------------------------------------

# 3. 可视化的实现
#--------------------------------------------------------------
acc = history.history['sparse_categorical_accuracy']
val_acc = history.history['val_sparse_categorical_accuracy']
loss = history.history['loss']
val_loss = history.history['val_loss']

plt.subplot(1, 2, 1)
plt.plot(acc, label='Training Accuracy')
plt.plot(val_acc, label='Validation Accuracy')
plt.title('Training and Validation Accuracy')
plt.legend()

plt.subplot(1, 2, 2)
plt.plot(loss, label='Training Loss')
plt.plot(val_loss, label='Validation Loss')
plt.title('Training and Validation Loss')
plt.legend()
plt.show()
#--------------------------------------------------------------
