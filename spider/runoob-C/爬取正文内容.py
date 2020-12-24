import xlrd
import urllib
import urllib.request
from bs4 import BeautifulSoup
import re
def main():
    book = xlrd.open_workbook('info.xls')
    sheet = book.sheets()[0]
    baseurllist = sheet.col_values(0,start_rowx=0,end_rowx=None)
    # alldatalist = []
    for baseurl in baseurllist:
        basehtml = askUrl(baseurl)
        datalist = getData(basehtml)
        # # alldatalist.append(datalist)
        # download(datalist)
        break

def askUrl(baseurl):
    print('正在向网站发出请求')
    head = {'User-Agent':'Mozilla/5.0 (Linux; Android 6.0; Nexus 5 Build/MRA58N) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/87.0.4280.88 Mobile Safari/537.36 Edg/87.0.664.57'}
    request = urllib.request.Request(baseurl, headers=head)
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

def getData(basehtml):
    print('正在解析得到的html')
    soup = BeautifulSoup(basehtml, 'html.parser')
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

if __name__ == '__main__':
    main()
    print('---写入完成---')