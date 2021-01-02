import tensorflow as tf
from tensorflow.keras.preprocessing.image import ImageDataGenerator

fashion_mnist = tf.keras.datasets.fashion_mnist
(x_train, y_train), (x_test, y_test) = fashion_mnist.load_data()
x_train, x_test = x_train/255.0, x_test/255.0

#--------------------------------------------------------------
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
#--------------------------------------------------------------

model = tf.keras.Sequential([
    tf.keras.layers.Flatten(),
    tf.keras.layers.Dense(128, activation='relu'),
    # tf.keras.layers.Dense(32, activation='sigmoid',kernel_regularizer=tf.keras.regularizers.l2()),
    tf.keras.layers.Dense(10, activation='softmax'),
])

model.compile(  optimizer='adam',
                loss=tf.keras.losses.SparseCategoricalCrossentropy(from_logits=False),
                metrics=['sparse_categorical_accuracy'])

# for i in range(32):

model.fit(x_train, y_train, batch_size=48, epochs=5, validation_data=(x_test,y_test), validation_freq=1)

#---------------------------------------------------------------
model.fit(img_gen_train.flow(x_train, y_train, batch_size=48), epochs=5, validation_data=(x_test,y_test), validation_freq=1)
#---------------------------------------------------------------

model.summary()