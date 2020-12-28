import tensorflow as tf
w = tf.Variable(tf.constant(5, dtype=tf.float32))
lr = 0.0002
epoch = 40000

for epoch in range(epoch):
    with tf.GradientTape() as tape:
        loss = tf.square(w+1)
    grads = tape.gradient(loss, w)

    w.assign_(lr * grads)
    print(f'{epoch} : {w.numpy()} : {loss}')