#!/usr/bin/python3
# 文件名：client.py

# 导入 socket、sys 模块
import socket
import sys

# for i in sys.argv:
#     print(i)
print('占位符')
# print(sys.argv[:])

# 创建 socket 对象

c = socket.socket(socket.AF_INET, socket.SOCK_STREAM) 
# 获取本地主机名
host = socket.gethostname() 
# 设置端口号
port = 9999
# 连接服务，指定主机和端口
c.connect((host, port))
# s.send(bytes('[1,2,3,4,5,6]'.encode('utf-8')))
# c.sendto(sys.argv[1].encode('utf-8'), (host, port))
c.sendto(sys.argv[0].encode('utf-8'), (host, port))
# 接收小于 1024 字节的数据
msg = c.recv(1000000).decode('utf-8')
if msg == "error":
    print("error")
else:
    print(str(msg))
    # for i in msg:
    #     print(i)
    # for i in list(msg):
    #     print(str(i))

c.close()


# for i in list(msg):
#     print(i)
# print(str(list(msg)))