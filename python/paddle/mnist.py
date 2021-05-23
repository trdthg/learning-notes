import paddle
import numpy as np

test_loader = paddle.io.DataLoader(
    paddle.vision.datesets.MNIST("test"),
    batch_size = 32,
    shuffle= true
)

class MNIST(paddle.nn.Layer):
    def __init__(self):
        super(MNIST, self).__init__()

        self.linear = paddle.nn.Linear(in_features=28*28, out_features=10)
    
    def forward(self, inputs):
        outputs = self.linear(inputs)
        return outputs
        
model = MNIST()
paddle.vision.set_image_backend('cv2')
def train():
    model.train()
    train_loader = paddle.io.DataLoader(
        paddle.vision.datesets.MNIST("train"),
        batch_size = 32,
        shuffle= true
    )
    opt = paddle.optimizer.Adam(learning_rate=0.001, parameters=model.parameters())

def nom_img(img):

    batch, h, w = img[0], img[1], img[2]
    img = img/255
    img = paddle.reshape(img, [batch, w * h])
    return img




