from bs4 import BeautifulSoup
file = open("D:/Project/Python/python爬虫/test/baidu.html",mode="rb")
file = file.read()
fileSource = BeautifulSoup(file,"html.parser")
# 只能找到第一个内容
print(fileSource.head.link)
print('------------------------------------------------------------------------------')
# 1.访问内容  字符串类型
print(fileSource.head.link.string)
# 2.访问属性  字典类型
print(fileSource.head.link.attrs)
print(fileSource.head.link.attrs['href'])
# 3.访问文档  就是整个文档
print(type(fileSource.head.link))
# 4.访问注释  类型与字符串不一样,但是是访问方法相同,切输出内容不包含注释
print('------------------------------------------------------------------------------')


# 上述方法能拿到内容,但是效率太低
# 下列方法可以将标签转化为列表,执行遍历操作
for child in fileSource.head.contents:
    print(child)
print('------------------------------------------------------------------------------')


# !!!查询!!!
# 1.字符串过滤  必须完全一样
t_list = fileSource.find_all('a')
for child in t_list:
    print (child)
print('__ '*50)
# 2.正则表达式搜索: 使用search()方法匹配内容
import re
t_list = fileSource.find_all(re.compile('a'))
for child in t_list:
    print (child)
print('------------------------------------------------------------------------------')
# 3.方法搜索: 传入一个函数,根据函数要求搜索
def name_is_exists(tag):
    return tag.has_attr("name")
t_list = fileSource.find_all(name_is_exists(fileSource.body))
for child in fileSource:
    print(child)
print('------------------------------------------------------------------------------')

# 包含特定元素
t_list = fileSource.find_all(id='head')
print(t_list)
t_list = fileSource.find_all(class_=False)
for child in t_list:
    print(child)
print('------------------------------------------------------------------------------')
# 包含特定文本
t_list = fileSource.find_all(text='弟弟')
t_list = fileSource.find_all(text=['弟弟','乌龟盘','冬瓜'])  
t_list = fileSource.find_all(text=re.compile('\d'), limit=3) #限制找的的数量
print(t_list)
print('------------------------------------------------------------------------------')


# !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!无敌的css选择器
t_list = fileSource.select("a[href='#']")  # 可以加# 或者. 和jQuery一样  
for child in t_list:
    print(child.get_text())  # get_text方法获取内容
    print(child.attrs["href"])  # 获取元素内容

t_list = fileSource.find_all('div', class_="item")


# 查找字符串
m = re.compile('AA')
p = m.search("aaRGIEAABGRJNAA")
print(p)
n = re.search('A','FaEFESGA')
print(n)
# 列出匹配的字符串
n = re.findall(re.compile("[A-Z0-5]+"),'FaA5682A547EFAESGA')
print(n)
# 替换字符串
print(re.sub('a', 'A', 'aaaaaaaaaAAAAAAAAAA'))  

n = "\aabb-\'"
n = r"\aabb-\'"  # 加上r能够防止被转义
print(n)