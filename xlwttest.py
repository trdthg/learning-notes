import xlwt
workbook = xlwt.Workbook(encoding='utf-8')
worksheet = workbook.add_sheet('aaa')
worksheet.write(0,0,'Hello World')
worksheet.write(0,1,'Hello Python')
worksheet.write(2,0,'Hello xlwt')
workbook.save('test.xls')