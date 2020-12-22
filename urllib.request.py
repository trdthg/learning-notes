import urllib.request
import urllib.parse
import urllib.error
import urllib.response
def line():
    print("------------------------------------------------------------------------")

#超时处理
try:
    #获取一个get请求
    response = urllib.request.urlopen("http://httpbin.org/get",timeout=0.01)#加上这句后,超时会报错)
    print(response.read().decode("utf-8"))
except urllib.error.URLError as e:
    print("time out !")
line()
#获取一个post请求
data = bytes(urllib.parse.urlencode({"hello":"world"}),encoding="utf-8")
response = urllib.request.urlopen("http://httpbin.org/post",data=data)
print(response.read().decode("utf-8"))
line()
#response的一些简单解析方法
response = urllib.request.urlopen("http://www.baidu.com")
  #爬取豆瓣会返回418,被识破了
print(response.status)
print(response.getheaders(),end="\n")
line()
print(response.getheader("Server"))

url = "http://www.douban.com"
headers = {"User-Agent":"Mozilla/5.0 (Linux; Android 6.0; Nexus 5 Build/MRA58N) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/86.0.4240.111 Mobile Safari/537.36 Edg/86.0.622.61"}
data = bytes(urllib.parse.urlencode({"name":"trdthg"}),encoding="utf-8")
req = urllib.request.Request(url=url,data=data,method="POST",headers=headers)
response = urllib.request.urlopen(req)
print(response.read().decode("utf-8"))