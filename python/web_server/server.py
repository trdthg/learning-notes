#!/usr/bin/python3
# 文件名：server.py

# 导入 socket、sys 模块
import socket
import sys
import json

import demjson

print("启动")

# 创建 socket 对象
s = socket.socket(
            socket.AF_INET, socket.SOCK_STREAM) 
# 获取本地主机名
host = socket.gethostname()

port = 9999

# 绑定端口号
s.bind((host, port))

# 设置最大连接数，超过后排队
s.listen(5)

while True:
    # 建立客户端连接
    c, c_addr = s.accept()      

    # msg = c.recv(100000)
    # print(msg)
    # # data, addr = serversocket.recvfrom(2048)   
    # msg = demjson.encode(demjson.decode(msg)).encode('utf-8').decode('unicode_escape')
    # print(msg, type(msg)) 
    # print("连接地址: %s" % str(c_addr))
    # msg = json.loads(msg)
    # print(msg, type(msg))
    data='欢迎访问菜鸟教程！'+ "\r\n"
    c.send(data.encode('utf-8'))
    c.close()
    # s.close()