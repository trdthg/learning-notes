# 接口文档(/▽＼)

## 注意事项
1. 大多数请求需要在header中添加token
2. 请求时不用加id是从token中解码得到
3. 有√的是暂时完成的
4. (new)表示新加的接口

## 更新记录
8.19 12:37
- √ (new)获取文章中所有 被划线句子及评论
- √ 获取杂志列表(不带content)
- √ 获取文章

## 个人

### √ 登录
1. req
```json
{
    "url":  "https://service-1v7iyl73-1306147581.bj.apigw.tencentcs.com/test/account_login",
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
    "url":  "https://service-1v7iyl73-1306147581.bj.apigw.tencentcs.com/test/account_register",
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
    "url":  "https://service-1v7iyl73-1306147581.bj.apigw.tencentcs.com/test/account_get_userinfo",
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
    "url":  "https://service-1v7iyl73-1306147581.bj.apigw.tencentcs.com/test/account_upload_avatar",
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
    "url":  "https://service-1v7iyl73-1306147581.bj.apigw.tencentcs.com/test/account_get_avatar",
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
    "url":  "https://service-1v7iyl73-1306147581.bj.apigw.tencentcs.com/test/account_get_toberead",
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
            "id": "",
            "nid": 1,
            "title": "",
            "url": ""
        }
    ]
}
```

### √ 获取摘记
1. req
```json
{
    "url":  "https://service-1v7iyl73-1306147581.bj.apigw.tencentcs.com/test/account_get_excerpt",
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
    "list": [
        {
            "article_id": 1,
            "comment": "笔记",
            "create_time": "Wed, 18 Aug 2021 13:34:44 GMT",
            "id": 1,
            "sentence": "摘录的原句",
            "user_id": 2
        }
    ]
}
```

### √ 获取足迹
1. req
```json
{
    "url":  "https://service-1v7iyl73-1306147581.bj.apigw.tencentcs.com/test/account_get_history",
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
    "list": [
        {
            "id": "",
            "nid": 1,
            "title": "",
            "url": ""
        }
    ]
}
```


### √ 获取收藏
1. req
```json
{
    "url":  "https://service-1v7iyl73-1306147581.bj.apigw.tencentcs.com/test/account_get_favorite",
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
    "list": [
        {
            "id": "",
            "nid": 1,
            "title": "",
            "url": ""
        }
    ]
}
```

## 新注册

### 获取分类标签
1. req
```json
{
    "url":  "https://service-1v7iyl73-1306147581.bj.apigw.tencentcs.com/test/account_get_tags",
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
    "url":  "https://service-1v7iyl73-1306147581.bj.apigw.tencentcs.com/test/account_upload_favorite_tags",
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

### √ (new)获取杂志列表(不带content)
1. req
```json
{
    "url":  "https://service-1v7iyl73-1306147581.bj.apigw.tencentcs.com/test/megazine_get_summarys",
    "method": "get",
    "header": {
        "Authorization": "token",
    },
    "params": {
        "category": "文化"
    }
}
```
2. res
```json
// 随机推20个现在是
{
    "code": 1,
    "list": [
        {
            "author": "ccc",
            "content": "ddd",
            "id": 1,  // 文章id
            "link": "fff",
            "summary": "bbb",
            "title": "aaa"
        }
    ]
}
```


### √ (new)获取文章
1. req
```json
{
    "url":  "https://service-1v7iyl73-1306147581.bj.apigw.tencentcs.com/test/article_get_article",
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
    "list": [
        {
            "author": "ccc",
            "content": "ddd",
            "create_time": null,
            "id": 1,
            "link": "fff",
            "summary": "bbb",
            "tag": null,
            "title": "aaa"
        }
    ]
}
```

## 评论

### √ 评论文章
1. req
```json
{
    "url":  "https://service-1v7iyl73-1306147581.bj.apigw.tencentcs.com/test/article_comment",
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
    "url":  "https://service-1v7iyl73-1306147581.bj.apigw.tencentcs.com/test/article_get_comment",
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

### √ (new)获取文章中所有 被划线句子及评论
1. req
```json
{
    "url":  "https://service-1v7iyl73-1306147581.bj.apigw.tencentcs.com/test/article_get_sentence_comment",
    "method": "get",
    "header": {
        "Authorization": "token",
    },
    "params": {
        "article_id": 1,
    }
}
```
2. res
```json
{
    "code": 1,
    "list": [
        {
            "comment": "对该句子的评头论足",
            "create_time": "Thu, 19 Aug 2021 01:51:36 GMT",
            "sentence": "文章中的某个句子",
            "user_id": 2,
            "username": "1"
        }
    ]
}
```

### √ 画线评论句子 
1. req
```json
{
    "url":  "https://service-1v7iyl73-1306147581.bj.apigw.tencentcs.com/test/sentence_comment",
    "method": "post",
    "header": {
        "Authorization": "token",
    },
    "body": {
        "sentence": "划线的句子内容",
        "comment": "评论",
        "article_id": 1,
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

### √ 获取被划线句子的评论
1. req
```json
{
    "url":  "https://service-1v7iyl73-1306147581.bj.apigw.tencentcs.com/test/sentence_get_comment",
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
            "comment": "评论",
            "create_time": "Sat, 14 Aug 2021 15:33:38 GMT",
            "father_id": 1,
            "user_id": 2,
            "username": "1"
        }
    ]
}
```

### √ 加入待读
1. req
```json
{
    "url":  "https://service-1v7iyl73-1306147581.bj.apigw.tencentcs.com/test/article_toberead",
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

### √ 新建摘记
```json
{
    "url":  "https://service-1v7iyl73-1306147581.bj.apigw.tencentcs.com/test/sentence_excerpt",
    "method": "get",
    "header": {
        "Authorization": "token",
    },
    "body": {
        "article_id": 1,
        "sentence": "摘录的原句",
        "comment": "笔记"
    }
}
```
2. res
```json
{
    "code": 1,
}
```

### √ 加入足迹
1. req
```json
{
    "url":  "https://service-1v7iyl73-1306147581.bj.apigw.tencentcs.com/test/article_history",
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

### √ 加入收藏
1. req
```json
{
    "url":  "https://service-1v7iyl73-1306147581.bj.apigw.tencentcs.com/test/article_favorite",
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

## 其他