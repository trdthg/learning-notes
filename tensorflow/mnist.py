import tensorflow as tf

# mnist = tf.keras.datasets.mnist
fashion_mnist = tf.keras.datasets.fashion_mnist
# (x_train, y_train), (x_test, y_test) = mnist.load_data()
(x_train, y_train), (x_test, y_test) = fashion_mnist.load_data()
x_train, x_test = x_train/255.0, x_test/255.0
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

model.summary()