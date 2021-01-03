from PIL import Image
import numpy as np
import tensorflow as tf

def main():
    model_save_path = './tensorflow/savedata/mnist1.ckpt'

    model = tf.keras.models.Sequential([
        tf.keras.layers.Flatten(),
        tf.keras.layers.Dense(128, activation='relu'),
        tf.keras.layers.Dense(64, activation='sigmoid',kernel_regularizer=tf.keras.regularizers.l2()),
        tf.keras.layers.Dense(10, activation='softmax')
    ])

    print('--------load weights--------')
    model.load_weights(model_save_path)

    preNum = int(input("Please input the number of test pictures:"))

    for i in range(preNum):
        image_path = input(f"PATH{i}:")
        img = Image.open('./mnist/' + image_path)
        img = img.resize((28,28), Image.ANTIALIAS)
        img_arr = np.array(img.convert('L'))

        for i in range(28):
            for j in range(28):
                if img_arr[i][j] < 200:
                    img_arr[i][j] = 255
                else: img_arr[i][j] = 0
        img_arr = img_arr / 255.0
        x_predict = img_arr[tf.newaxis, ...]
        result = model.predict(x_predict)
        pred = tf.argmax(result, axis=1)
        print('\n')
        tf.print(pred)
    print('---------------over------------------')

if __name__ == '__main__':
    main()