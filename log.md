# 日记

## 1月

### 1月14日

#### 1.  Java 处理JSON

```java
 @RequestMapping(value="/aaa",  method = RequestMethod.POST)
    @ResponseBody
    public JSONObject aaa(@RequestBody JSONObject user) {
        // 一.读取json数据
        {
            "num": 1234567,
            "pwd": 689753,
            "arr": [
                {"name":"张三", "age":12},
                {"name":"李四", "age":13},
                {"name":"王五", "age":14}
            ]
        }
        // 1.普通数据
        Integer num = user.get("num")
        // 2.列表嵌套json  
        ArrayList<Map> mmm = (ArrayList) user.get("arr");
        for (Map mm : mmm) {
            System.out.println(mm.get("name"));
        }
        
        // 二. 创建JSONObject实例并返回
        //1. 普通数据
        JSONObject user_ = new JSONObject();
        user_.put("num", user.get("num"));
        //2. 列表
        JSONObject info = new JSONObject();
        info.put("num", user.get("num"));    //创建{}
        JSONArray arr = new JSONArray();     
        arr.add(user__);                     // 创建列表arr[],并把info添加到arr中 => [{},{}]
        user_.put("套娃", arr);              // { key:val, key: [{}, {}]}
        return user_;
    }
```

#### 2.  ArrayList

1. 添加:  add()    
2. 取值:  get( int index )  
3. 修改: set( int index,  Object obj)
4. 删除: remove(int index)
5. 计算大小:  list.size()方法   

| [addAll()](https://www.runoob.com/java/java-arraylist-addall.html) | 添加集合中的所有元素到 arraylist 中           |
| ------------------------------------------------------------ | --------------------------------------------- |
| [clear()](https://www.runoob.com/java/java-arraylist-clear.html) | 删除 arraylist 中的所有元素                   |
| [clone()](https://www.runoob.com/java/java-arraylist-clone.html) | 复制一份 arraylist                            |
| [contains()](https://www.runoob.com/java/java-arraylist-contains.html) | 判断元素是否在 arraylist                      |
| [indexOf()](https://www.runoob.com/java/java-arraylist-indexof.html) | 返回 arraylist 中元素的索引值                 |
| [removeAll()](https://www.runoob.com/java/java-arraylist-removeall.html) | 删除存在于指定集合中的 arraylist 里的所有元素 |
| [remove()](https://www.runoob.com/java/java-arraylist-remove.html) | 删除 arraylist 里的单个元素                   |
| [isEmpty()](https://www.runoob.com/java/java-arraylist-isempty.html) | 判断 arraylist 是否为空                       |
| [subList()](https://www.runoob.com/java/java-arraylist-sublist.html) | 截取部分 arraylist 的元素                     |
| [sort()](https://www.runoob.com/java/java-arraylist-sort.html) | 对 arraylist 元素进行排序                     |
| [toArray()](https://www.runoob.com/java/java-arraylist-toarray.html) | 将 arraylist 转换为数组                       |
| [ensureCapacity](https://www.runoob.com/java/java-arraylist-surecapacity.html)() | 设置指定容量大小的 arraylist                  |
| [lastIndexOf()](https://www.runoob.com/java/java-arraylist-lastindexof.html) | 返回指定元素在 arraylist 中最后一次出现的位置 |
| [retainAll()](https://www.runoob.com/java/java-arraylist-retainall.html) | 保留 arraylist 中在指定集合中也存在的那些元素 |
| [containsAll()](https://www.runoob.com/java/java-arraylist-containsall.html) | 查看 arraylist 是否包含指定集合中的所有元素   |
| [trimToSize()](https://www.runoob.com/java/java-arraylist-trimtosize.html) | 将 arraylist 中的容量调整为数组中的元素个数   |
| [removeRange()](https://www.runoob.com/java/java-arraylist-removerange.html) | 删除 arraylist 中指定索引之间存在的元素       |
| [replaceAll()](https://www.runoob.com/java/java-arraylist-replaceall.html) | 将给定的操作内容替换掉数组中每一个元素        |
| [removeIf()](https://www.runoob.com/java/java-arraylist-removeif.html) | 删除所有满足特定条件的 arraylist 元素         |



#### 3. HashMap

创建Map:    HashMap<String, String> map = **new** HashMap<String, String>();

1. 添加 :  map.put("key" : "value" )

2. 获取 :  map.get(int index)

3. 删除 :  map.remove(int index)

4. 迭代方法

   ```java
   // 方法1    map.keySet()返回键集合   使用map.get(key)获取value
   for (Integer i : Sites.keySet()) {
       System.out.println("key: " + i + " value: " + Sites.get(i));
   }
   // 方法2    map.values()返回值集合   
   for(String value: Sites.values()) {
       System.out.print(value + ", ");
   }
   ```

| [clear()](https://www.runoob.com/java/java-hashmap-clear.html) | 删除 hashMap 中的所有键/值对                                 |
| ------------------------------------------------------------ | ------------------------------------------------------------ |
| [clone()](https://www.runoob.com/java/java-hashmap-clone.html) | 复制一份 hashMap                                             |
| [isEmpty()](https://www.runoob.com/java/java-hashmap-isempty.html) | 判断 hashMap 是否为空                                        |
| [size()](https://www.runoob.com/java/java-hashmap-size.html) | 计算 hashMap 中键/值对的数量                                 |
| [put()](https://www.runoob.com/java/java-hashmap-put.html)   | 将键/值对添加到 hashMap 中                                   |
| [putAll()](https://www.runoob.com/java/java-hashmap-putall.html) | 将所有键/值对添加到 hashMap 中                               |
| [putIfAbsent()](https://www.runoob.com/java/java-hashmap-putifabsent.html) | 如果 hashMap 中不存在指定的键，则将指定的键/值对插入到 hashMap 中。 |
| [remove()](https://www.runoob.com/java/java-hashmap-remove.html) | 删除 hashMap 中指定键 key 的映射关系                         |
| [containsKey()](https://www.runoob.com/java/java-hashmap-containskey.html) | 检查 hashMap 中是否存在指定的 key 对应的映射关系。           |
| [containsValue()](https://www.runoob.com/java/java-hashmap-containsvalue.html) | 检查 hashMap 中是否存在指定的 value 对应的映射关系。         |
| [replace()](https://www.runoob.com/java/java-hashmap-replace.html) | 替换 hashMap 中是指定的 key 对应的 value。                   |
| [replaceAll()](https://www.runoob.com/java/java-hashmap-replaceall.html) | 将 hashMap 中的所有映射关系替换成给定的函数所执行的结果。    |
| [get()](https://www.runoob.com/java/java-hashmap-get.html)   | 获取指定 key 对应对 value                                    |
| [getOrDefault()](https://www.runoob.com/java/java-hashmap-getordefault.html) | 获取指定 key 对应对 value，如果找不到 key ，则返回设置的默认值 |
| [forEach()](https://www.runoob.com/java/java-hashmap-foreach.html) | 对 hashMap 中的每个映射执行指定的操作。                      |
| [entrySet()](https://www.runoob.com/java/java-hashmap-entryset.html) | 返回 hashMap 中所有映射项的集合集合视图。                    |
| [keySet](https://www.runoob.com/java/java-hashmap-keyset.html)() | 返回 hashMap 中所有 key 组成的集合视图。                     |
| [values()](https://www.runoob.com/java/java-hashmap-values.html) | 返回 hashMap 中存在的所有 value 值。                         |
| [merge()](https://www.runoob.com/java/java-hashmap-merge.html) | 添加键值对到 hashMap 中                                      |
| [compute()](https://www.runoob.com/java/java-hashmap-compute.html) | 对 hashMap 中指定 key 的值进行重新计算                       |
| [computeIfAbsent()](https://www.runoob.com/java/java-hashmap-computeifabsent.html) | 对 hashMap 中指定 key 的值进行重新计算，如果不存在这个 key，则添加到 hasMap 中 |
| [computeIfPresent()](https://www.runoob.com/java/java-hashmap-computeifpresent.html) | 对 hashMap 中指定 key 的值进行重新计算，前提是该 key 存在于 hashMap 中。 |

### 1月15日

#### 1. 神经八股复习

```python
# 1. 引入数据集
fashion_mnist = tf.keras.datasets.fashion_mnist
# (x_train, y_train), (x_test, y_test) = mnist.load_data()
(x_train, y_train), (x_test, y_test) = fashion_mnist.load_data()
x_train, x_test = x_train/255.0, x_test/255.0
# 2. 搭建神经网络
model = tf.keras.Sequential([
    tf.keras.layers.Flatten(),
    tf.keras.layers.Dense(128, activation='relu'),
    # tf.keras.layers.Dense(32, activation='sigmoid',kernel_regularizer=tf.keras.regularizers.l2()),
    tf.keras.layers.Dense(10, activation='softmax'),
])
# 3. 选择 优化器 | 损失函数 | 以及metrics
model.compile(  optimizer='adam',
                loss=tf.keras.losses.SparseCategoricalCrossentropy(from_logits=False),
                metrics=['sparse_categorical_accuracy'])
# PS: metrics可选:    y_ = y_train  y = x_train * w + b
        1. 'accuracy'  y_和y都是数值
        2. 'categorical_accuracy' y_和y都是独热码
        3. 'sparsecategorical_accuracy' y_是数值y是独热码
# 4. 执行训练过程
model.fit(
    x_train, y_train, 
    batch_size=48, epochs=5, 
    validation_data=(x_test,y_test),  #(1)任选一个 选择测试集
    validation_split=                 #(2)任选一个 不选择测试集, 从训练集中划分一块作为测试集
    validation_freq=1   # 多少epoch测试一次
)
# 5. 打印网络结构和参数统计
model.summary()
```

#### 2. numpy方法

```python
1.   reshape(数据, 形状)  =>   x_train = np.reshape(x_train,  (len(x_train), 1, 5))
2.   np.random.shuffle()   # 列表打乱重排, 将x_train和y_train用相同的随机数重排
```

#### 3. RNN期望的输入形状

```python
# 以字母预测为例    a->b b->c c->d d->e e->a
x_train = np.reshape(x_train, (len(x_train), 1, 5))    
'''
三个参数分别是 (送入样本数 循环核时间展开步数 每个时间步输入特征数)
这里每次输入一个字母 返回一个预测字母(输入一次就给出预测结果)  所以循环核时间展开步数为1
输入特征是独热码[1,0,0,0,0]有五个值,所以每个时间步输入特征数为5 
'''
```

### 1月16日

#### 1. Embedding编码

```python
# 在sequential中搭建神经网络时先进行embedding编码
# 1. 将x_train变成embedding期待的形状
x_train = np.reshape(x_train, (len(x_train), 1)  # (样本数, 时间展开步数)
# 2. 搭建神经网络
model = tf.keras.Sequential([
    Embedding(5,2),   # (需要表示的结果数量, 编码维度(几个数字能表示一个结果))
    SimpleRNN(3),
    Dense(5, activation='softmax')
])
```

#### 2. spring MVC

```java
    /// 1. 普通传参
	@RequestMapping(value="/index3", method = RequestMethod.POST, params={"name", "id=10"})
    // ！！！注意！！！ id 的类型已经被自动完成类型转换
    //0 public String index3(String name, int id) {    正常
    //(1) 如果不想用name 和 id 作为参数  需要进行参数绑定
    //(2)（1）public String index3(String str, int age) {    报错
    //(2)（2） public String index3(String str, Integer age) {   不报错 但值均为Null
    public String index3(@RequestParam("name") String str,@RequestParam("id") int age) {
        System.out.println("执行了带有参数的POST请求");
        // return "您传递的name是" + name + " id是" + id;
        return "您传递的name是" + str + " id是" + age;
    }
    /// 2. rest风格的传参  必须进行参数映射
    @RequestMapping(value="/rest/{name}/{id}", method = RequestMethod.POST)
    public String rest(@PathVariable("name") String name,@PathVariable("id") int id) {
        return "您传递的name是" + name + " id是" + id;
    }

    /// 3. 取出cookie信息
    @RequestMapping("/cookie")
    public String cookie (@CookieValue(value = "JSESSIONID") String sessionId) {
        return "SESSIONID: " + sessionId ;
    }

```

#### 3. SpringBoot(水)

```yml
server:
  port: 8081  # 默认端口
```

#### 4. pandas读取列

```python
data = pd.read_csv('./pandas/test.csv')
# 1. loc单行
# 2. iloc[1:2,3:4] 按索引 行和列
# 3. iloc[[1,2,3], [2,3]] 第1,2,3行的第2,3列
data_name_and_score = data.iloc[1:2, 1:3].values  # iloc方法 [行, 列] 都是前闭后开  结果以二维数组表示
# 4. 写入csv/excel
df.to_csv('a.csv') 
df.to_excel('a.xlsx', sheet_name='sheet1') 
```

#### 5. numpy

```python
# 1. np.asarray() 可以将数据转为numpy格式
# 2. data.flatten() 可以将数据拉为一维
# 3. reshaape(data, (1,-1))   (数据, 形状) 不必多说
```

### 1月17日

#### 1. Sequential参数

```python
# 1. RNN
SimpleRNN(80, return_sequences=True),  # 两层都是RNN时,前一层要加上return_sequences=True
# 2. Dropout
Dropout(0.2),  # 随即扔掉一些神经元, 防止过拟合, 可以先设为0, 逐渐调大, 找到最优值
```

#### 2. cp_callbacks参数

```python
cp_callback = tf.keras.callbacks.ModelCheckPoint(
    filepath = checkpoint_save_path,
    save_weights_only = True,
    save_best_only = True,
    monitor = 'var_loss', # 指定需要监测的值
)
```

#### 3. model.fit参数

```python
# 都用来描述验证集, 与测试集不同
validation_data=(x_test, y_test),
validation_freq=1,
'''
其实验证集是从训练集中抽取出来用于调参的，
而测试集是和训练集无交集的，用于测试所选参数用于该模型的效果的，这个还是不要弄错了。。。
在Keras中，验证集的划分只要在fit函数里设置validation_split的值就好了，这个对应了取训练集中百分之几的数据出来当做验证集。
'''
```

1. > 训练集（train set） —— 用于模型拟合的数据样本。在训练过程中对训练误差进行梯度下降
   >
   > 作用:训练的权重参数。

2. > 验证集（validation set）—— 是模型训练过程中单独留出的样本集，
   >
   > 作用:调整模型的超参数
   > 		验证集可以用在训练的过程中，一般在训练时，几个epoch结束后跑一次验证集看看效果。(验证得太频繁会影响训练速度)这样做的第一个好处是，可以及时发现模型或者参数的问题，比如模型在验证集上发散啦、出现很奇怪的结果啦(如无穷大)、mAP不增长或者增长很慢啦等等情况，这时可以及时终止训练，重新调参或者调整模型，而不需要等到训练结束。另外一个好处是验证模型的泛化能力，如果在验证集上的效果比训练集上差很多，就该考虑模型是否过拟合了。同时，还可以通过验证集对比不同的模型。在一般的神经网络中， 我们用验证数据集去寻找最优的网络深度（number of hidden layers)，或者决定反向传播算法的停止点或者在神经网络中选择隐藏层神经元的数量； 
   >         由于验证集是用来”训练”超参数的，尽管验证集的误差通常会比训练集误差小，一般来说验证集比较小会低估泛化误差。所有超参数优化完成之后，泛化误差可能会通过测试集来估计。   
   >         在普通的机器学习中常用的交叉验证（Cross Validation) 就是把训练数据集本身再细分成不同的验证数据集去训练模型。

3. > 测试集 —— 用来评估模最终模型的泛化能力。但不能作为调参、选择特征等算法相关的选择的依据。
   >
   > 作用: 只是验证

#### 4. 归一化操作

```python
from sklearn.preprocessing import MinMaxScaler
motai = pd.read_csv('./tensorflow/SH600519.csv')
train_set = motai.iloc[:2426-300,2:3].values
test_set = motai.iloc[2426-300:,2:3].values
# !! 归一化 !!
sc = MinMaxScaler(feature_range=(0,1))  # 定义归一化, 选定范围到(0-1)间
train_set_scaled = sc.fit_transform(train_set)
# fit 求得训练集固有属性(如平均值, 最大值, 方差等), transform对训练集进行归一化
test_set = sc.transform(test_set)
# 利用训练集的属性对测试集进行归一化
```

#### 5. TensorBoard使用

```python
# 1. 设置路径和文件名
log_dir="logs/fit/" + datetime.datetime.now().strftime("%Y%m%d-%H%M%S")
# 2. 添加 tf.keras.callback.TensorBoard 回调可确保创建和存储日志
tensorboard_callback = tf.keras.callbacks.TensorBoard(log_dir=log_dir, histogram_freq=1)
# 3. callbacks中加入
callbacks = [cp_callback,tensorboard_callback]
# 4. 控制台运行  (需要将tensorboard加入环境变量  例如:C:\Users\YMZ\anaconda3\envs\tf2\Scripts  )
tensorboard --logdir logs/fit
```

### 1月18日

#### 1.  傻子预测股市

1. 输入 : 60 天的开盘价
2. 输出 : 61天的开盘价
3. 把预测的61天当作输入值 , 重复上述实验
4. 结果 : 曲线趋于平坦, 没用

```python
i = 500
data60_ori = maotai.iloc[:1213,2:3].values
def pre(i, data60_ori):
    i -= 1
    if i > 0:
        data60 = data60_ori
        data60 = sc.transform(data60)
        readyfortest = []
        readyfortest.append(data60[len(data60)-60:len(data60),0])
        readyfortest = np.array(readyfortest)
        readyfortest = np.reshape(readyfortest, (readyfortest.shape[0], 60, 1))
        preprice = model.predict(readyfortest)
        preprice = sc.inverse_transform(preprice)
        data60_ori = np.vstack((data60_ori, preprice))
        pre(i, data60_ori)
    else: 
        print('------over------')
        print(data60_ori.shape)
        plt.plot(all_set, c='r', label='Real Price')
        plt.plot(data60_ori, c='b', label='Predict Price')
        plt.plot(test1, c='y', label='Predict Price')
        plt.legend()
        plt.show()
```

#### 2. pandas 删除列

```python
# 在数据预处理中，需要删除dataframe的一列的话，可以使用下面的方法。
train = train.drop(['A'], axis=1)
# 其中axis=1代表的是要删除一列，而不是一行。
```

#### 3. pandas 解析时间

```python
trips = pd.read_csv('./data/trips.csv',encoding='gbk') 
# 1. 转换格式
trips['进站时间'] = pd.to_datetime(trips['进站时间'],format="%Y/%m/%d %H:%M")
# 2. 解析
year = trips['某一列'].dt.year  # 获取一列
year = trips['某一列'][int index].year # 只获取一个
```

#### 4. pandas筛选数据

```python
# 1. ==
print( trips.loc[ trips['进站时间'].dt.month.isin([4,5]) ]) 
# 2. is in [] 
print( trips.loc[ trips['进站时间'].dt.month.isin([2,3,4]) ])
# 3.  & | !
注意只有一个 & 或 |
# 4. 筛选结果计数 (两种都行)
print( trips.loc[ trips['进站时间'].dt.month.isin([2,3,5,6,7]) ].count())  
print( trips.loc[ trips['进站时间'].dt.month.isin([2,3,5,6,7]) ]['进站时间'].count())  


```

### 1月19日

#### 1. pandas行列操作

```python
# 1. 判断是星期几
trips['dayofweek'] = trips['进站时间'].dt.dayofweek
# 2. 新增行/列
trips['new_col'] = '0'
trips.loc['new_row'] = '0'
# 3. 遍历行
for index, row in trips.iterrows():
# 4. 替换某格数据
trips.iloc[index,10] = row_[1]
```

#### 2. pandas日期(完整)

```python
pandas 日期时间数据的分割提取操作
import pandas as pd
import numpy as np
import time
s=time.time()
data_2019=pd.read_excel('d:\\data\\abc.xlsx')
data=data_2019[['卡号','交易时间']]
data['日期'] =data_2019['交易时间'].dt.date
data['时间'] =data_2019['交易时间'].dt.time
data['年'] = data_2019['交易时间'].dt.year
data['季节'] = data_2019['交易时间'].dt.quarter
data['月'] = data_2019['交易时间'].dt.month
data['周']=data_2019['交易时间'].dt.week
data['日'] = data_2019['交易时间'].dt.day
data['小时'] =data_2019['交易时间'].dt.hour
data['分钟'] =data_2019['交易时间'].dt.minute
data['秒'] = data_2019['交易时间'].dt.second
data['一年第几天'] =data_2019['交易时间'].dt.dayofyear
data['一年第几周'] = data_2019['交易时间'].dt.weekofyear
data['一周第几天'] = data_2019['交易时间'].dt.dayofweek
data['一个月含有多少天'] = data_2019['交易时间'].dt.days_in_month
data['星期名称'] =data_2019['交易时间'].dt.weekday_name
print(data)
data.to_excel('d:\\data\\abcsss.xlsx')
```

#### 3. Series对象

```python
# Series 常用属性和方法
获取数据的值, 使用 values方法
获取索引的值, 使用 index 方法
获取每对索引的值, 使用 items 方法
```

### 1月20日

#### 1. 获取某月天数

```python
import calendar
res = calendar.monthrange(2020,5)
print(res[1])
```

#### 2. 纵向显示x轴坐标

```python
df = pd.DataFrame(pd.read_csv('./sta_flow_by_day.csv'))

plt.xticks(rotation=270)
plt.bar([f'{month}.{day}' for month, day in zip(np.array(df['month']), np.array(df['day']))], np.array(df['flow']))
plt.show()
```

#### 3. pandas筛选注意

```python
# 不同条件一定要加括号
df = df.loc[(df['sta'] == 'Sta1') & (df['month']==2)]
```

### 1月21日

#### 1. 分割线 (水)

```python
print('#-&-$-@-%-'*13)
```

#### 2. 防止无key

> ###### try except真好用

```python
for key in stas.keys():
    month = stas[key]
    for sta in month.keys():
        try:
            big_dict[sta][key] = month[sta]      
        except:
            try:
                big_dict_gun[sta][key] = month[sta]
            except:
                big_dict_gun[sta] = {}
```

#### 3. onehot应用

```python
# 2. 把route列转为onehot编码
enc_route = sklearn.preprocessing.OneHotEncoder(sparse=False) # Key here is sparse=False!
route_onehot = enc_route.fit_transform(np.array(list(df['route'])).reshape(len(df['route']),1))
print(df['route'].shape)
print(route_onehot)

# fit_transform = fit + transform
# 之后可以用 enc_route.transform() 编码测试集
```

### 1月22日

#### 1. 反归一化(1.17)

```python
preprice = model.predict(readyfortest)
preprice = sc.inverse_transform(preprice)
```

#### 2. pandas排序

```python
#默认为升序
df_3 = df.sort_values(by=['day','flow'])
```



### 1.23日~ 1.27日

#### 1.神经网络经验总结

```python

# 1. 模型隐藏层层数设置
1. 在神经网络中，当且仅当数据非线性分离时才需要隐藏层！
2. 对于一般简单的数据集，一两层隐藏层通常就足够了。但对于涉及时间序列或计算机视觉的复杂数据集，则需要额外增加层数。单层神经网络只能用于表示线性分离函数，也就是非常简单的问题，比如分类问题中的两个类可以用一条直线整齐地分开。
3. 一般规律
没有隐藏层：仅能够表示线性可分函数或决策
隐藏层数=1：可以拟合任何“包含从一个有限空间到另一个有限空间的连续映射”的函数
隐藏层数=2：搭配适当的激活函数可以表示任意精度的任意决策边界，并且可以拟合任何精度的任何平滑映射
隐藏层数>2：多出来的隐藏层可以学习复杂的描述（某种自动特征工程）
# 2. 模型隐藏层神经元个数设置
1、隐藏单元的数量不应该超过输入层中单元的两倍
2、隐藏单元的大小应该介于输入单元和输出单元之间
3、神经元的数量应捕获输入数据集方差的70~90%
# 3. 注意输入训练集的形状
# 4. 注意输入层特征的输入顺序
与输出层关联性强的放前面
有些关联性弱的特征加入会严重扰乱准确性 比如day
```



## 3月

### 3月6日

##### 1. pickle模块

```python

#pickle提供了一个简单的持久化功能。可以将对象以文件的形式存放在磁盘上。
'''pickle模块只能在python中使用，python中几乎所有的数据类型（列表，字典，集合，类等）都可以用pickle来序列化，
pickle序列化后的数据，可读性差，人一般无法识别。'''

1. pickle.dump(obj, file[, protocol])
　　序列化对象，并将结果数据流写入到文件对象中。参数protocol是序列化模式，默认值为0，表示以文本的形式序列化。		protocol的值还可以是1或2，表示以二进制的形式序列化。
1. pickle.load(file)
　　反序列化对象。将文件中的数据解析为一个Python对象。
　　
#其中要注意的是，在load(file)的时候，要让python能够找到类的定义，否则会报错：
```

##### 2. networks.from_dict_of_lists

```python
# 读取字典对象, 并制成graph
dol= {0:[1]} # single edge (0,1)
G=nx.from_dict_of_lists(dol)
```

