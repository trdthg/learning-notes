import os
import sys
import time

from flask import Blueprint, render_template, request, session, redirect, current_app, flash, url_for

sys.path.append(".")
from utils.db import SQLHelper
from utils.wrappers import *

sentence = Blueprint('sentence', __name__, url_prefix='/sentence')


@sentence.route('/comment',methods=["POST"])
@is_login
def comment(user_id):
    try: 
        nowtime = time.strftime("%Y-%m-%d %H:%M:%S", time.localtime()) 
        article_id = request.get_json()["article_id"]
        sentence = request.get_json()["sentence"]
        comment = request.get_json()["comment"]
        father_id = request.get_json().get('comment_id', 0)
        SQLHelper().insert('''
            INSERT INTO sentence(sentence, article_id)
            VALUES (%s, %s)''', (
            sentence, article_id))
        # sentence_id = SQLHelper().execute("SELECT LAST_INSERT_ID()").get('LAST_INSERT_ID()')
        # print(sentence_id)
        sentence_id = SQLHelper().execute("SELECT max(id) from sentence").get("max(id)")
        res = SQLHelper().insert('''
            INSERT INTO comment_sentence(user_id, comment, sentence_id, father_id, create_time) 
            VALUES (%s, %s, %s, %s, %s)''', (
            user_id, comment, sentence_id, father_id, nowtime))
        return {'code': 1}
    except:
        return { 'code': 0, 'msg': '评论失败' }

@sentence.route('/get_comment',methods=["GET"])
@is_login
def get_comment(user_id):
    try: 
        sentence_id = request.args["sentence_id"]
        res = SQLHelper().fetch_all('''
            SELECT c.user_id, u.username, c.comment, c.father_id, c.create_time
            FROM (comment_sentence c, user u)
            WHERE (c.sentence_id = %s AND u.id = c.user_id)
            LIMIT 3 OFFSET 0''', (
                sentence_id
            ))
        return {'code': 1, 'list': res}
    except:
        return { 'code': 0, 'msg': '获取评论失败' }











