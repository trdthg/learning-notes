import pandas  as pd
import numpy as np

# 直接读取
data = pd.read_csv('./pandas/test.csv',header=1) # 加上header=1能忽略第一行
print(data)
data_name_and_score = data.iloc[:, 3:4].values

# 转换后读取 (可操作性更强)
df = pd.DataFrame(pd.read_csv('./pandas/test.csv'))
print(df.info(),df.columns, df.dtypes, df['姓名'].dtype, df.values)
# 提取 行/列 数据
df2 = df.iloc[:,1:3]
'''
loc单行
iloc[1:2,3:4] 按索引 行和列
iloc[[1,2,3], [2,3]] 第1,2,3行的第2,3列
'''
print(df.loc[2], df.iloc[1:2,2:3], )
df = np.reshape(df.iloc[:,2:3], (len(df)))
arr2 = np.asarray(df2)
print(df)
arr = np.asarray(df)
print(arr.flatten())
print(arr2.reshape(1,-1))