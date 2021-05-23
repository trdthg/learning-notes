import os
import numpy as np
import tensorflow as tf
import matplotlib.pyplot as plt
from tensorflow.keras.layers import Dense, SimpleRNN 

input_word = "abcde"
w_to_id = {
    'a': 0,
    'b': 1,
    'c': 2,
    'd': 3,
    'e': 4,
}
id_to_onehot = {
    0: [1.,0.,0.,0.,0.],
    1: [0.,1.,0.,0.,0.],
    2: [0.,0.,1.,0.,0.],
    3: [0.,0.,0.,1.,0.],
    4: [0.,0.,0.,0.,1.],
}
w_to_onehot = {
    'a': [1.,0.,0.,0.,0.],
    'b': [0.,1.,0.,0.,0.],
    'c': [0.,0.,1.,0.,0.],
    'd': [0.,0.,0.,1.,0.],
    'e': [0.,0.,0.,0.,1.],
}
x_train = [id_to_onehot[w_to_id['a']], id_to_onehot[w_to_id['b']], id_to_onehot[w_to_id['c']], id_to_onehot[w_to_id['d']], id_to_onehot[w_to_id['e']]]
x_train = [w_to_onehot['a'],w_to_onehot['b'],w_to_onehot['c'],w_to_onehot['d'],w_to_onehot['e']]
y_train = [w_to_id['b'], w_to_id['c'], w_to_id['d'], w_to_id['e'], w_to_id['a']]

np.random.seed(7)
np.random.shuffle(x_train)
np.random.seed(7)
np.random.shuffle(y_train)
tf.random.set_seed(7)

x_train = np.reshape(x_train, (len(x_train), 1, 5))
y_train = np.array(y_train)

model = tf.keras.Sequential([
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
#     # monitor = 'loss'
# )

history = model.fit(
    x_train, y_train, 
    batch_size=32, epochs=5, 
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
    alphabet = [id_to_onehot[w_to_id[alphabet]]]
    alphabet = np.reshape(alphabet, (1,1,5))
    result = model.predict([alphabet])
    pred = tf.argmax(result, axis=1)
    pred = int(pred)
    tf.print(alphabet1 + '->' + imput_word[pred])

