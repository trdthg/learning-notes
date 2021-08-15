import os
import sys
import time

from nanoid import generate
from werkzeug.utils import secure_filename
from flask import Blueprint, render_template, request, session, redirect, current_app, flash, url_for

sys.path.append(".")
from utils.db import SQLHelper
from utils.tokenUtil import TokenHelper
from utils.wrappers import *

article = Blueprint('article', __name__, url_prefix='/article')

@article.route('/toberead',methods=["GET"])
@is_login
def favorite(user_id):
    try: 
        nowtime = time.strftime("%Y-%m-%d %H:%M:%S", time.localtime()) 
        article_id = request.args.get("article_id")
        res = SQLHelper().insert('''
            insert into user_toberead(user_id, article_id, create_time) 
            select %s, %s, %s from DUAL where not exists 
            (select id from user_toberead where user_id = %s and article_id = %s)''', (
            user_id, article_id, nowtime, user_id, article_id))
        return {'code': 1}
    except:
        return { 'code': 0, 'msg': '加入待读失败' }

@article.route('/comment',methods=["POST"])
@is_login
def comment(user_id):
    try: 
        nowtime = time.strftime("%Y-%m-%d %H:%M:%S", time.localtime()) 
        article_id = request.get_json()["article_id"]
        comment = request.get_json()["comment"]
        res = SQLHelper().insert('''
            INSERT INTO comment_article(user_id, comment, article_id, create_time) 
            VALUES (%s, %s, %s, %s)''', (
            user_id, comment, article_id, nowtime))
        return {'code': 1}
    except:
        return { 'code': 0, 'msg': '评论失败' }

@article.route('/get_comment',methods=["GET"])
@is_login
def get_comment(user_id):
    try: 
        article_id = request.args["article_id"]
        res = SQLHelper().fetch_all('''
            SELECT c.user_id, u.username, c.comment, c.create_time
            FROM (comment_article c, user u)
            WHERE (c.article_id = %s AND u.id = c.user_id)''', (
                article_id
            ))
        return {'code': 1, 'list': res}
    except:
        return { 'code': 0, 'msg': '获取评论失败' }






















