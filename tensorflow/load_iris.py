from sklearn import datasets
from pandas import DataFrame
import pandas as pd

# iris数据集
x_data = datasets.load_iris().data
y_data = datasets.load_iris().target
print(x_data)
print(y_data)

x_data = DataFrame(x_data, columns=['花萼长','花萼宽','花瓣长','花瓣宽'])
pd.set_option('display.unicode.east_asian_width', True)
print(x_data)

x_data['类别'] = y_data
print(x_data)

# 手写数字数据集
x_data = datasets.load_digits().data
y_data = datasets.load_digits().target
print(x_data)
print(y_data)
# pd.set_option('display.unicode.east_asian_width', True)
# print(x_data)