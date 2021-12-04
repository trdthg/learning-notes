#导入opencv模块
import cv2
#捕捉帧，笔记本摄像头设置为0即可
capture: cv2.VideoCapture() = cv2.VideoCapture(0)
import time
a = cv2.imread("./test.jpg")
print(a)
cv2.namedWindow("Image")
cv2.imshow("Image", a)
cv2.waitKey(delay=0)
# Close all windows
cv2.destroyAllWindows()

# while(True):
#     print(1)
#     ret, frame = capture.read()
#     time.sleep(1)
#     print(frame[0])
# #     #显示窗口第一个参数是窗口名，第二个参数是内容
#     cv2.imshow('frame', frame[0])
# #     if cv2.waitKey(1) == ord('q'):#按Q退出
# #         break
