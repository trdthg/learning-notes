import redis

# 1. 基础
# decode_responses 解码为字符串
r = redis.StrictRedis(host='localhost', port=6379, decode_responses=True)
r.set('id', 1)
r.set('name', 'xiaoming')
id = r.get('id')
name = r.get('name')
print(id, type(id))
print(name, type(name))

# 2. 连接池
pool = redis.ConnectionPool(host='localhost', port=6379, decode_responses=True)
r = redis.Redis(connection_pool=pool)
r.set('name', 'zhangsan')
name = r.get('name')

# 3. 基本命令

## String
# ex - 过期时间（秒）
# px - 过期时间（毫秒）
# nx - 如果设置为True，则只有name不存在时，当前set操作才执行
# xx - 如果设置为True，则只有name存在时，当前set操作才执行
r.set('name', 'zhangsan', ex=3)

r.mset({ # 批量设置
    'name1': 'zhangsan',
    'name2': 'lisi'
})

names = r.mget(['name1', 'name2']) # 批量获取

slice_ = r.getrange('name1', 0, -1) # 切片， 双闭区间

r.setrange('name1', 1, 'kkkkk')  # 从指定位置开始替换， 若超过则向后添加
print(r.get('name1'))
length = r.strlen('name1') # 返回值的字节长度 一个汉字3个字节

r.incr('id', amount=1) #自增name对应的值，当 name 不存在时，则创建 name＝amount
r.incrbyfloat('id', amount=1.0) # 浮点数
r.decr('id', amount=1)
# 应用场景 – 页面点击数

r.append('name1', 'hhh') # 值后追加

print(r.get('name1'))

## Hash
r.hset('zhangsan', 'id', 1)
r.hset('zhangsan', 'score', 70)
r.hset('lisi', mapping={'id': 2, 'score': 90})
val = r.hget('zhangsan', 'score')
val_list = r.hmget('zhangsan', 'id', 'score')
keys = r.hkeys('lisi')
vals = r.hvals('lisi')
print(keys, vals)
print(val, val_list)

dict_ = r.hgetall('zhangsan')
hlen = r.hlen('lisi')
r.hdel('lisi', 'id')
is_exist = r.hexists('lisi', 'id')
r.hincrby('lisi', 'id', amount=1)  # 可负数
r.hincrbyfloat('lisi', 'id', amount=1.3)  # 可负数
print(dict_, hlen, is_exist)

r.hset('scan', mapping={i: i for i in range(67)})
print(r.hscan('scan', cursor=0, count=1))
print(r.hscan('scan', cursor=1))
for item in r.hscan_iter('scan'):
    print(item)

## List
r.lpush("list1", 11, 22, 33)
print(r.lrange('list1', 0, 2))

r.rpush("list2", 11, 22, 33)  # 表示从右向左操作
print(r.llen("list2"))  # 列表长度
print(r.lrange("list2", 0, 3))  # 切片取出值，范围是索引号0-3
