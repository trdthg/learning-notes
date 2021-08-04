# 接口文档(/▽＼)

## 注意事项
1. 大多数请求需要在header中添加token
2. 请求时不用加id是从token中解码得到
3. 一堆params是空的未来可能填上参数
4. 说不定"xxx_id"最后都会被简化为"id"
5. 因为写数据库结构思路的不同, 该api极有可能随时修改
4. 本md非常随意, 请不要较真ψ(｀∇´)ψ

## 个人

### 未登录

#### 登录
1. res
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
2. res
```json
{
    "code": 1,
    "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJpc3N1ZSBieSBjbGFzcyBUb2tlbiIsInN1YiI6InN1YiBmb3IgYWxsIHVzZXIgd2hlbiB0cnlpbmcgdG8gbG9naW4iLCJpYXQiOjE2MjgwNTkyNTIuMjM3NDQ1LCJ1c2VybmFtZSI6InJvb3QyIiwiZXhwIjoxNjI4MDYwODI0LjQwOTg3fQ.RnF99g9FK1psdAorqxtKfpFFhbGxRagXLu3aX2odypc",
    "user_id": 1,
}
```

#### 注册
1. res
```json
{
    "url":  "http://127.0.0.1:5000/account/register",
    "method": "post",
    "header": {
        "Authorization": "token",
    },
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
    "user_id": 1,
}
```

### 登录

#### 获取个人资料
1. res
```json
{
    "url":  "http://127.0.0.1:5000/account/get_userinfo",
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
    "username": "",
    "email": "",
    "avatar": "",
}
```

#### 上传头像
1. res
```json
{
    "url":  "http://127.0.0.1:5000/account/upload_avatar",
    "method": "post",
    "header": {
        "Authorization": "token",
    },
    "params": {
        "avatar": "sssss.jpg"
    }
}
```
2. res
```json
{
    "code": 1,
    "user_id": 1,
}
```

#### 获取待读
1. res
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

#### 获取摘记
1. res
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

#### 获取足迹
1. res
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

#### 获取收藏
1. res
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

## 新注册用户

### 获取分类标签
1. res
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
1. res
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

### 评论展示

#### 获取句子
1. res
```json
{
    "url":  "http://127.0.0.1:5000/comment/get_sentence",
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
            "sentence_id": 1,
            "sentence": "sssssssssssssssssss",
            "origin_link": "xxxxxxx",
        },
        {
            "sentence_id": 1,
            "sentence": "sssssssssssssssssss",
            "origin_link": "xxxxxxx",
        },
    ]
}
```

### 作者社区
无[dog]

## 今日杂志

### 获取杂志
1. res
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

### 文章

#### 对句子的评论 
1. res
```json
{
    "url":  "http://127.0.0.1:5000/comment/sentence",
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

#### 获取评论
1. res
```json
{
    "url":  "http://127.0.0.1:5000/comment/get_sentence_comment",
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
            "sentence_id": 1,
            "comment": "sssssssssssssssssss",
            "user_id": 1,
            "username": "asdadawdawd",
            "avatar": "xxx.jpg",
        },
        {
            "sentence_id": 1,
            "comment": "sssssssssssssssssss",
            "user_id": 1,
            "username": "asdadawdawd",
            "avatar": "xxx.jpg",
        },
    ]
}
```