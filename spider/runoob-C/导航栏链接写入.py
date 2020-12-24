import urllib
import urllib.request
from bs4 import BeautifulSoup
import re
import requests
import xlwt
def main():
    # 爬取图片的url
    basebaseurl = 'https://www.runoob.com/cprogramming/c-tutorial.html'
    html = askUrl(basebaseurl)
    basealllist = getbaseurllist(html)
    # 把这些链接写到excel里
    book(basealllist)
    
    # filepath = 'D:/Project/python/python爬虫/source/菜鸟教程C导航栏/'
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
    # print(html)
    print('请求完毕')
    return html

def getbaseurllist(html):
    # modellink = re.compile(r'<a href="(.*)" tar')
	# modellink = re.compile(r'<img class=\"list-img lazyload\" src=\"https\:\/\/static\.nyahentai\.pw\/img\/list-loading\.svg" data-src="(.*)" onerror="javascript:this\.src=\'https:\/\/i0\.nyacdn\.com\/galleries\/1712249\/1\.png\';this\.onerror = null" alt="\[劇毒少女 (ke-ta)\] 夜のキセキ-Tanzanite- (東方Project) \[DL版\] - Picture 1">')

    print('正在解析得到的html')
    soup = BeautifulSoup(html, 'html.parser')
    # print(soup)

    print('正在获取文本列表')
    namelist = []
    t_list = soup.select('div > a[target="_top"]')  # 可以加# 或者. 和jQuery一样  
    for child in t_list:
        namelist.append((child.get_text()).strip())  # get_text方法获取内容
    
    print('正在获取链接')
    item = soup.find_all('div', class_="design",id="leftcolumn", limit=36)
    # namelist = re.findall(r'C [\u4E00-\u9FFF a-z 0-9 ]*', item)
    item = str(item)
    item = re.sub('href="', 'http://www.runoob.com', item)
    linklist = re.findall(r'http://www\.runoob\.com/cprogramming/c-.*?\.html', item)
    
    print('准备下载')

    # for link in linklist:
    #     print(link)
    # for link in namelist:
    #     print(link)
    return [linklist,namelist]

def downloadImg(imgurlist, filepath):
    i = 1
    for url in imgurlist:
        print(f'正在下载第{i}张图片')
        if i<=77:
            i += 1
            continue
        file = filepath + f'{i}.jpg'
        url = requests.get(url)
        with open(file, 'wb') as code:
            code.write(url.content)
        i += 1

def book(basealllist):
    workbook = xlwt.Workbook(encoding='utf-8')
    worksheet = workbook.add_sheet('c_sheet1')
    worksheet.col(0).width = 15000
    worksheet.col(1).width = 6000
    i = 0
    for baseurl in basealllist[0]:
        worksheet.write(i, 0, baseurl)
        i += 1
        if i == 36:
            break
    i = 0
    for name in basealllist[1]:
        worksheet.write(i, 1, name)
        i += 1
        if i == 36:
            break
    
    workbook.save('./info.xls')

if __name__ == '__main__':
    main()
    print('---下载完成,爬取完毕---')