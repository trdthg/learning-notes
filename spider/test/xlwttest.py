import xlwt
workbook = xlwt.Workbook(encoding='utf-8')
worksheet = workbook.add_sheet('aaa')
for i in range(0,100):
    worksheet.write(i,0,'Hello World')
    worksheet.write(i,1,'Hello Python')
    worksheet.write(i,2,'Hello xlwt')
workbook.save('test.xls')