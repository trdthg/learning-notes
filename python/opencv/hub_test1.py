

# import paddlehub as hub

# lac = hub.Module(name="lac")
# test_text = ["今天是个好天气。"]

# results = lac.cut(text=test_text, use_gpu=False, batch_size=1, return_tag=True)
# print(results)
# #{'word': ['今天', '是', '个', '好天气', '。'], 'tag': ['TIME', 'v', 'q', 'n', 'w']}

import paddle
import cv2
import paddlehub as hub

if __name__ == '__main__':
    capture = cv2.VideoCapture(0)
    capture.set(3,140)
    # model = hub.Module(name='unet_cityscapes')
    face_detector = hub.Module(name="ultra_light_fast_generic_face_detector_1mb_640")

    #循环显示帧
    i = 1
    while(True):
        print(i)
        i += 1
        ret, frame = capture.read()
        #显示窗口第一个参数是窗口名，第二个参数是内容
        cv2.imshow('frame', frame)  
        if cv2.waitKey(1) == ord('q'):#按Q退出
            break
        img = frame
        # model.predict(images=[img], visualization=True)
        result = face_detector.face_detection(images=[img], visualization=True)
    
