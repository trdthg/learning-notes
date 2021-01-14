import urllib
import urllib.request
import urllib.error
baseurl = "http://localhost:8081/index/aaa"
data = {
    'num':123456,
    'pwd':123
}
print('正在向网站发出请求')
head = {'User-Agent':'Mozilla/5.0 (Linux; Android 6.0; Nexus 5 Build/MRA58N) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/87.0.4280.88 Mobile Safari/537.36 Edg/87.0.664.57'}
request = urllib.request.Request(baseurl,headers=head)
html = ''
try:
    response = urllib.request.urlopen(request)
    html = response.read().decode('utf-8')
except urllib.error.URLError as e:
    if hasattr(e,"code"):
        print(e.code)
    if hasattr(e,"reason"):
        print(e.reason)
print('请求完毕')
print(html)