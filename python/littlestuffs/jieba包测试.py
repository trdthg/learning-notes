import jieba
import jieba.analyse
import jieba.posseg as pseg
txt = "屏幕wwwwww上次的个体和融合和"
a = jieba.cut(txt)
print(" ".join(a))


a = jieba.analyse.textrank(txt,topK=20, withWeight=True, allowPOS=('n','nr','ns'))
if a==[] :
    print("1")
else:
    print(a)
    for list in a:
        for i in list:
            if type(i) == type(""):
                # print(i)
                pass
        