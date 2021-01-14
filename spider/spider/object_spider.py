
import urllib
import urllib.request
from bs4 import BeautifulSoup
import re
import requests
import os
import time
import multiprocessing as mp

class Spider(): 

    # def __init__(self):
        # pass
        

    def main(self, baseurl, filepath):

        self.create_folder(filepath)
        html = self.askUrl(baseurl)
        imgurllist = self.getImgurl(html)
        
        # 普通爬 太慢了
        # downloadImg(imgurllist, filepath)
        # 多进程爬 NB
        self.multiDownload(imgurllist, filepath)

    def create_folder(self, filepath):
        os.makedirs(filepath)

    def askUrl(self,baseurl):
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

    def getImgurl(self, html):
        print('正在解析html')

            # 方法1  htmlpaser解析 ，正则表达式
            # modellink = re.compile(r'data-src="(.*)"')
            # modellink = re.compile(r'<img class=\"list-img lazyload\" src=\"https\:\/\/static\.nyahentai\.pw\/img\/list-loading\.svg" data-src="(.*)" onerror="javascript:this\.src=\'https:\/\/i0\.nyacdn\.com\/galleries\/1712249\/1\.png\';this\.onerror = null" alt="\[劇毒少女 (ke-ta)\] 夜のキセキ-Tanzanite- (東方Project) \[DL版\] - Picture 1">')
            # 解析得到的html
            # soup = BeautifulSoup(html, 'html.parser')
            # print(soup)
            # item = soup.find_all('section', id="image-container")
            # item = str(item)
            # linklist = re.findall(r'data-src="(.*)" on', item)

        '''!!!可以使用set()去重!!!'''

        # lxml解析  css获取
        soup = BeautifulSoup(html,'lxml')  #速度更快， 需要安装C语言库
        item = soup.select('#page-container > #image-container > img')
        # item = soup.find_all('section', {'id':'page-container','class':'....'} )
        print('正在获取链接')
        linklist = [l['data-src'] for l in item]
        # for it in item :
        #     print (it)
        # for it in linklist :
        #     print(it)
        # for link in linklist:
        #     print(link)




        print('准备下载')
        return linklist

    def downloadImg(self, imgurlist, filepath):
        i = 1
        for url in imgurlist:
            time.sleep(1)
            print(f'正在下载第{i}张图片')
            #---------------------------------------------------
            # if i<=108:
            #     i += 1
            #     continue
            #---------------------------------------------------
            file = filepath + f'{i}.jpg'
            r = requests.get(url, stream=True)
            with open(file, 'wb') as f:
                # f.write(r.content)
                for chunk in r.iter_content(chunk_size=32):
                    f.write(chunk)
            #---------------------------------------------------
            # if i==92: #缺第几张就填几
            #     break
            #---------------------------------------------------

            i += 1
        
    def download(self, imgurl):
        #---------------------------------------------------
        # if i<=108:
        #     i += 1
        #     continue
        #---------------------------------------------------
        print(f'正在下载第{imgurl[1]}张图片')
        filepath = f'./spider/source/hhh/'
        filepath = imgurl[2]
        file = filepath + f'{imgurl[1]}.jpg'
        r = requests.get(imgurl[0], stream=True)
        with open(file, 'wb') as f:
            # f.write(r.content)
            for chunk in r.iter_content(chunk_size=32):
                f.write(chunk)
        #---------------------------------------------------
        # if i==92: #缺第几张就填几
        #     break
        #---------------------------------------------------


    def multiDownload(self, imgurllist, filepath):
        pool = mp.Pool(processes=10)
        a = []
        for i in range(len(imgurllist)):
            a.append(filepath)
        result = pool.map(self.download, zip(imgurllist,range(1,len(imgurllist)+1), a))

if __name__ == '__main__':
    spider1 = Spider()
    # spider1.main('https://zh.nyahentai.xyz/g/283000/list2/', './spider/source/魔女的夜宴/')
    spider2 = Spider()
    # spider2.main('https://zha.doghentai.com/g/259422/list2/', './spider/source/anecdoteC95/')

    print('---下载完成,爬取完毕---')