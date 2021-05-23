import bs4
import re
import time
import xlwt
import sqlite3
import urllib.request
import urllib.error
from bs4 import BeautifulSoup
import requests

def main():
    baseurl = "https://movie.douban.com/top250?start="
    urllist = getDate(baseurl)
    
    #3. 保存数据
    # savePath = "./豆瓣电影Top250.xls"
    # saveData(savePath,urllist)
    downloadData(urllist)

modellink = re.compile(r'<img.*src="(.*?)".*?>', re.S)

def getDate(baseurl):
    #--爬取网页--
    
    #1. 爬取网页
    #datalist = getDate(baseurl)
    datalist = []
    k=1
    for i in range(0,10):
        url = baseurl + str(i*25)
        html = askURL(url)    #保存获取到的网页源码
        #2. 逐一解析数据
        # print(html)
        soup = BeautifulSoup(html, 'html.parser')
        
        for item in soup.find_all("div",class_="item"):
            # print(item)
            data = []
            item = str(item)
            print(f"正在保存第{k}条链接")
            link = re.findall(modellink, item)
            datalist.append(link[0])    

            k+=1

    
    return datalist

#得到一个特定的URL网页内容
def askURL(url):
    head = {    #模拟浏览器头部信息, 向豆瓣服务器发送消息
            "User-Agent":"Mozilla/5.0 (Linux; Android 6.0; Nexus 5 Build/MRA58N) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/86.0.4240.111 Mobile Safari/537.36 Edg/86.0.622.61"
        }
                #用户代理,表示告诉网站我们是什么机器
    request = urllib.request.Request(url,headers=head)
    html=""
    try:
        response = urllib.request.urlopen(request)
        html = response.read().decode("utf-8")
        # print(html)
    except urllib.error.URLError as e:
        if hasattr(e,"code"):
            print(e.code)

        if hasattr(e,"reason"):
            print(e.reason)
    return html

#保存数据
def saveData(savePath,urllist):
    print("save...")
    book = xlwt.Workbook(encoding='utf-8', style_compression=0)
    sheet = book.add_sheet('图片链接', cell_overwrite_ok=True)
    col = ('电影链接','还没想好放啥')
    for i in range(0,2):
        sheet.write(0,i,col[i])
    for i in range(0,250):
        print('第%d条'%i)
        sheet.write(i,0,urllist[i])
    book.save(savePath)

def downloadData(urllist):
    i=1
    for url in urllist:
        print(f'正在下载第{i}张图片')
        file = f"D:/Project/python/python爬虫/source/test/{i}.jpg"
        r = requests.get(url)
        with open(file, "wb") as code:
            code.write(r.content)
        i += 1


if __name__ == "__main__":
    main()
    print('爬取完毕')