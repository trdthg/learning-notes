
import urllib
import urllib.request
from bs4 import BeautifulSoup
import re
import requests
def main():
    # 爬取图片的url
    baseurl = 'https://www.pixiv.net'
    html = askUrl(baseurl)
    imgurllist = getImgurl(html)
    filepath = 'D:/Project/python/python爬虫/source/pixiv_12.23/'
    # downloadImg(imgurllist, filepath)
def askUrl(baseurl):
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
    return html

def getImgurl(html):
    print('正在解析html')
    modellink = re.compile(r'src="(.*)"')
	# modellink = re.compile(r'<img class=\"list-img lazyload\" src=\"https\:\/\/static\.nyahentai\.pw\/img\/list-loading\.svg" data-src="(.*)" onerror="javascript:this\.src=\'https:\/\/i0\.nyacdn\.com\/galleries\/1712249\/1\.png\';this\.onerror = null" alt="\[劇毒少女 (ke-ta)\] 夜のキセキ-Tanzanite- (東方Project) \[DL版\] - Picture 1">')
    # 解析得到的html
    soup = BeautifulSoup(html, 'html.parser')
    # print(soup)
    item = soup.find_all("div", class_="rp5asc-9 jDfaKM")
    # print(item)
    item = str(item)
    print('正在获取链接')
    linklist = re.findall(r'src="(.*)" alt=', item)
    print('准备下载')

    # for link in linklist:
    #     print(link)
    return linklist

def downloadImg(imgurlist, filepath):
    i = 1
    for url in imgurlist:
        print(f'正在下载第{i}张图片')
        # if i<=74:
        #     i += 1
        #     continue
        file = filepath + f'{i}.jpg'
        url = requests.get(url)
        with open(file, 'wb') as code:
            code.write(url.content)
        # if i==75:
        #     break
        i += 1
    


if __name__ == '__main__':
    main()
    print('---下载完成,爬取完毕---')