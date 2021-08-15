# 接口文档(/▽＼)

## 注意事项
1. 大多数请求需要在header中添加token
2. 请求时不用加id是从token中解码得到
3. 一堆params是空的未来可能填上参数
4. 说不定"xxx_id"最后都会被简化为"id"
5. 因为写数据库结构思路的不同, 该api极有可能随时修改
4. 本md非常随意, 请不要较真ψ(｀∇´)ψ

## 个人

### √ 登录
1. req
```json
{
    "url":  "http://127.0.0.1:5000/account/login",
    "method": "post",
    "body": {
        "username": "root",
        "password": "000000",
    }
}
```
2. req
```json
{
    "code": 1,
    "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJpc3N1ZSBieSBjbGFzcyBUb2tlbiIsInN1YiI6InN1YiBmb3IgYWxsIHVzZXIgd2hlbiB0cnlpbmcgdG8gbG9naW4iLCJpYXQiOjE2MjgwNTkyNTIuMjM3NDQ1LCJ1c2VybmFtZSI6InJvb3QyIiwiZXhwIjoxNjI4MDYwODI0LjQwOTg3fQ.RnF99g9FK1psdAorqxtKfpFFhbGxRagXLu3aX2odypc",
    "user_id": 1,
}
```

### √ 注册
1. req
```json
{
    "url":  "http://127.0.0.1:5000/account/register",
    "method": "post",
    "body": {
        "username": "root",
        "password": "000000",
        "email": "xxxxx@xxx.xx",
    }
}
```
2. res
```json
{
    "code": 1,
    "msg": "success",
    "user_id": 1,
}
```

### √ 获取个人资料
1. req
```json
{
    "url":  "http://127.0.0.1:5000/account/get_userinfo",
    "method": "get",
    "header": {
        "Authorization": "token",
    },
}
```
2. res
```json
{
    "code": 1,
    "userinfo": {
        "avatar": "avatar_2_xrfuO61aqbTrR8ilkk.jpg",
        "email": "1@qq.com",
        "username": "1"
    }
}
```

### √ 上传头像
1. req
```json
{
    "url":  "http://127.0.0.1:5000/account/upload_avatar",
    "method": "post",
    "header": {
        "Authorization": "token",
    },
    "form": {
        "avatar": "sssss.jpg"
    }
}
```
2. res
```json
{
    "code": 1,
}
```

### √ 获取头像
1. req
```json
{
    "url":  "http://127.0.0.1:5000/account/get_avatar",
    "method": "post",
    "header": {
        "Authorization": "token",
    },
}
```
2. res
```json
{
    "code": "1",
    "avatar_url": "/static/avatar/avatar_2_xrfuO61aqbTrR8ilkk.jpg"
}
```

### √ 获取待读
1. req
```json
{
    "url":  "http://127.0.0.1:5000/account/get_toberead",
    "method": "get",
    "header": {
        "Authorization": "token",
    },
    "params": {
        
    }
    
}
```
2. res
```json
{
    "code": 1,
    "list": [
        {
            "article_id": "",
            "title": "",
        },
        {
            "article_id": "",
            "title": "",
        }
    ]
}
```

### 获取摘记
1. req
```json
{
    "url":  "http://127.0.0.1:5000/account/get_excerpt",
    "method": "get",
    "header": {
        "Authorization": "token",
    },
    "params": {
        
    }
    
}
```
2. res
```json
{
    "code": 1,
    "list": [
        {
            "excerpt_id": "",
            "except": "",
            "notes": "",
            "article_origin_link": "",
            "article_title": "",
        },
        {
            "excerpt_id": "",
            "except": "",
            "notes": "",
            "article_origin_link": "",
            "article_title": "",
        }
    ]
}
```

### 获取足迹
1. req
```json
{
    "url":  "http://127.0.0.1:5000/account/get_history",
    "method": "get",
    "header": {
        "Authorization": "token",
    },
    "params": {
        
    }
    
}
```
2. res
```json
{
    "code": 1,
    "list": [
        {
            "article_id": "",
            "title": "",
        },
        {
            "article_id": "",
            "title": "",
        }
    ]
}
```

### 获取收藏
1. req
```json
{
    "url":  "http://127.0.0.1:5000/account/get_favorite",
    "method": "get",
    "header": {
        "Authorization": "token",
    },
    "params": {
        
    }
    
}
```
2. res
```json
{
    "code": 1,
    "list": [
        {
            "article_id": "",
            "title": "",
        },
        {
            "article_id": "",
            "title": "",
        }
    ]
}
```

## 新注册

### 获取分类标签
1. req
```json
{
    "url":  "http://127.0.0.1:5000/account/get_tags",
    "method": "get",
    "header": {
        "Authorization": "token",
    },
    "params": {
        
    }
}
```
2. res
```json
{
    "code": 1,
    "list": [
        {
            "tag_id": 1,
            "tag_name": "sssss",
        },
        {
            "tag_id": 1,
            "tag_name": "sssss",
        },
    ]
}
```

### 标签选择
1. req
```json
{
    "url":  "http://127.0.0.1:5000/account/upload_favorite_tags",
    "method": "post",
    "header": {
        "Authorization": "token",
    },
    "body": {
        "tag_ids": [1, 2, 3, 4],
    }
}
```
2. res
```json
{
    "code": 1,
}
```

## 发现

### 作者社区
无[dog]

## 杂志

### 获取杂志
1. req
```json
{
    "url":  "http://127.0.0.1:5000/today/get_magazine",
    "method": "get",
    "header": {
        "Authorization": "token",
    },
    "params": {
        "type": 1, // 这个type是为了区分 个人杂志和分类杂志(maybe)等不同类型的杂志
        // 说不定要分成两个接口, 两个数据库
    }
}
```
2. res
```json
{
    "code": 1,
    "magazine_id": 1,
    "cover": "xxxxxx.jpg",
    "menu": [
        {
            "article_id": 1,
            "title": "sss",
            "summary": "sss", // 概要
            "origin_link": "sssssssss",
        },
        {
            "article_id": 1,
            "title": "sss",
            "summary": "sss", // 概要
            "origin_link": "sssssssss",
        },
    ]
}
```

### √ 文章加入待读
1. req
```json
{
    "url":  "http://127.0.0.1:5000/article/toberead",
    "method": "get",
    "header": {
        "Authorization": "token",
    },
    "params": {
        "article_id": 1
    }
}
```
2. res
```json
{
    "code": 1,
}
```

## 评论

### √ 评论文章
1. req
```json
{
    "url":  "http://127.0.0.1:5000/article/comment",
    "method": "post",
    "header": {
        "Authorization": "token",
    },
    "body": {
        "article_id": "",
        "comment": "ssssssssssssssssssssssssssssssssssssssssssssssssssssss",
    }
}
```
2. res
```json
{
    "code": 1,
}
```

### √ 获取文章评论
1. req
```json
{
    "url":  "http://127.0.0.1:5000/article/get_comment",
    "method": "get",
    "header": {
        "Authorization": "token",
    },
    "params": {
        "sarticle_id": 1,
    }
}
```
2. res
```json
{
    "code": 1,
    "list": [
        {
            "comment": "一条评论",
            "create_time": "Sat, 14 Aug 2021 13:24:33 GMT",
            "user_id": 2,
            "username": "1"
        }
    ]
}
```


### √ 评论句子 
1. req
```json
{
    "url":  "http://127.0.0.1:5000/sentence/comment",
    "method": "post",
    "header": {
        "Authorization": "token",
    },
    "body": {
        "sentence": "sssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssss",
        "comment": "",
        "user_id": 1,
        "article_id": "",
    }
}
```
2. res
```json
{
    "code": 1,
    "sentence_id": 1,
}
```

### √ 获取句子评论
1. req
```json
{
    "url":  "http://127.0.0.1:5000/sentence/get_comment",
    "method": "get",
    "header": {
        "Authorization": "token",
    },
    "params": {
        "sentence_id": 1,
    }
}
```
2. res
```json
{
    "code": 1,
    "list": [
        {
            "comment": "对该句子的评论",
            "create_time": "Sat, 14 Aug 2021 15:33:38 GMT",
            "father_id": 1,
            "user_id": 2,
            "username": "1"
        }
    ]
}
```

## 其他