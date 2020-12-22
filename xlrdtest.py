import xlrd

book = xlrd.open_workbook('D:/Project/Python/python爬虫/runoob-C/info.xls')
# 获取表名
names = book.sheet_names()
# print(names)

# 获取sheet的三种方式
sheet = book.sheets()[0]
sheet = book.sheet_by_index(sheetx=0)
sheet = book.sheet_by_name(sheet_name='导航栏链接')
# print(type(sheet))

# 获取行数/列数
n_rows=sheet.nrows  # 获取该sheet中的有效行数
n_cols=sheet.ncols  # 获取该sheet中的有效列数
# print(n_rows)
# print(n_cols)

# 获取单行/单列所有格子的 !!!对象!!!
row_list=sheet.row(rowx=1)  # 返回某行中所有的单元格对象组成的列表
col_list=sheet.col(colx=0) # 返回某列中所有的单元格对象组成的列表
# print(row_list)
# print(col_list)

# 某行或列的信息列表
row_data=sheet.row_values(0,start_colx=0,end_colx=None)
cols_data=sheet.col_values(1,start_rowx=0,end_rowx=None)
# print(row_data)
# print(cols_data)

row_lenth=sheet.row_len(0)  # 返回某行的有效单元格长度
print(row_lenth)

# excel工作表的单元格操作
row_col=sheet.cell(rowx=0,colx=0)  # 返回单元格对象
row_col_data=sheet.cell_value(rowx=0,colx=0) # 返回单元格中的数据
