import numpy as np
num_inputs = 1
num_examples = 100
F = np.random.randint(1, 100, (num_examples, num_inputs)).astype("float32")
print(F[:,0])
M = 0.05
labels = F[:, 0] * M
labels = labels + np.random.normal(0, 0.01, labels.shape[0])
labels = labels.astype("float32")
# 扩展维度
labels = np.expand_dims(labels, axis=-1)

import paddle
trainn_datas = paddle.to_tensor(F)
y_true = paddle.to_tensor(labels)
print(trainn_datas)
model = paddle.nn.Linear(
    in_features = 1, 
    out_features = 1, 
    weight_attr=None, 
    bias_attr=None, 
    name=None
)

mse_loss = paddle.nn.MSELoss()

lr = paddle.optimizer.lr.CosineAnnealingDecay(
    learning_rate=0.1,
    T_max=10
)

opt = paddle.optimizer.Adam(
    learning_rate = lr,
    parameters = model.parameters(),
    weight_decay =  paddle.regularizer.L2Decay(0.01),
)

for i in range(100):
    y_predict = model(trainn_datas)
    loss = mse_loss(y_predict, y_true)
    loss.backward()
    opt.step()
    opt.clear_grad()

print(loss.numpy())