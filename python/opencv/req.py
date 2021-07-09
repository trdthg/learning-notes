import requests
import json
import cv2
import base64
import paddlehub as hub
import time
def cv2_to_base64(image):
    data = cv2.imencode('.jpg', image)[1]
    return base64.b64encode(data.tostring()).decode('utf8')

    

capture = cv2.VideoCapture(0)
#循环显示帧
i = 1
while(True):
    print(i)
    i += 1
    ret, frame = capture.read()
    # 发送HTTP请求
    data = {'images':[cv2_to_base64(frame)]}
    headers = {"Content-type": "application/json"}
    url = "http://127.0.0.1:8866/predict/ultra_light_fast_generic_face_detector_1mb_320"
    r = requests.post(url=url, headers=headers, data=json.dumps(data))
    # 打印预测结果
    try:
        print(r.json()["results"])
        result = r.json()["results"][0]["data"][0]
        cv2.rectangle(frame, (int(result["left"]), int(result["bottom"])), (int(result["right"]), int(result["top"])), (255, 0, 0), 2)
    except: 
        pass
    cv2.imshow('frame', frame) 
    if cv2.waitKey(1) == ord('q'):#按Q退出
        break

    # [{'data': [{'bottom': 274.1576843261719, 'confidence': 0.9995881915092468, 'left': 139.28065490722656, 'right': 283.00067138671875, 'top': 27.361862182617188}]}]