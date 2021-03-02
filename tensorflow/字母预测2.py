import numpy as np
import tensorflow as tf
from tensorflow.keras.layers import Dense, SimpleRNN, Embedding
import matplotlib.pyplot as plt
import os

input_word = 'abcde'

w_to_id = {'a':0, 'b':1, 'c':2, 'd':3, 'e':4}
x_train = [w_to_id['a'], w_to_id['b'], w_to_id['c'], w_to_id['d'], w_to_id['e']]
y_train = [w_to_id['b'], w_to_id['c'], w_to_id['d'], w_to_id['e'], w_to_id['a']]
np.random.seed(7)
np.random.shuffle(x_train)
np.random.seed(7)
np.random.shuffle(x_train)
tf.random.set_seed(7)
x_train = np.reshape(x_train, (len(x_train), 1))
print(x_train.shape)
print(x_train.shape)
print(x_train.shape)
print(x_train.shape)
y_train = np.array(y_train)
model = tf.keras.Sequential([
    Embedding(5, 2),
    SimpleRNN(3),
    Dense(5, activation='softmax')
])



model.compile(
    optimizer = tf.keras.optimizers.Adam(0.01),
    loss = tf.keras.losses.SparseCategoricalCrossentropy(from_logits=False),
    # metrics = ['categorical_accuracy'],
    metrics = ['sparse_categorical_accuracy'],
)

checkpoint_save_path = "./checkpoint/rnn1-15.ckpt"

# if os.path.exists(checkpoint_save_path + '.index'):
#     print('------------load the model-----------')
#     model.load_weights(checkpoint_save_path)

# cp_callback = tf.keras.callbacks.ModelCheckpoint(
#     filepath = checkpoint_save_path,
#     save_weights_onle = True,
#     save_best_only = True,
#     monitor = 'loss'
# )

history = model.fit(
    x_train, y_train, 
    batch_size=32, epochs=10, 
    # callbacks=[cp_callback]
)

model.summary()

acc = history.history['sparse_categorical_accuracy']
loss = history.history['loss']

plt.subplot(1,2,1)
plt.plot(acc, label='Acc')
plt.legend()
plt.subplot(1,2,2)
plt.plot(loss, label='Loss')
plt.legend()
plt.show()
for i in range(2):
    preNum = int(input("Input test alphabet:"))
    alphabet = [w_to_id[alphabet]]
    alphabet = np.reshape(alphabet, (1,1))
    result = model.predict([alphabet])
    pred = tf.argmax(result, axis=1)
    pred = int(pred)
    tf.print(alphabet1 + '->' + imput_word[pred])









