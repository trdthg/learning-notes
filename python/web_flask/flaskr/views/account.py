import os
import sys
import time

from nanoid import generate
from werkzeug.utils import secure_filename
from flask import Blueprint, render_template, request, session, redirect, current_app, flash, url_for
from flask_uploads import UploadSet, configure_uploads, IMAGES

sys.path.append(".")
from utils.db import SQLHelper
from utils.tokenUtil import TokenHelper
from utils.wrappers import *

account = Blueprint('account', __name__, url_prefix='/account')

@account.route('/login',methods=["POST"])
def login():
    info = request.get_json()
    user_id = SQLHelper().fetch_one("select id from user where username = %s and password = %s", (info['username'], info['password']))
    if user_id:
        username = info['username']
        session[username] = user_id
        session.permanent = True
        token = TokenHelper().encrpyt_token(username)
    return {'status': 200, 'user': user_id, 'token': token}

@account.route('/register',methods=['POST'])
def register():
    info = request.get_json()
    nowtime = time.strftime("%Y-%m-%d %H:%M:%S", time.localtime()) 
    # newuser = SQLHelper().insert("insert into user(username, password, email, create_time, update_time) values (%s, %s, %s, %s, %s)", (
    #     info['username'], info['password'], info['email'], nowtime, nowtime
    # ))
    newuser = SQLHelper().insert('''
        insert into user(username, password, email, create_time, update_time) 
        select %s, %s, %s, %s, %s from DUAL where not exists 
        (select id from user where username = %s)''', (
        info['username'], info['password'], info['email'], nowtime, nowtime, info['username'],
    ))
    return {'status': 200, "newuser": newuser}


def allowed_file(filename):
    return '.' in filename and \
filename.rsplit('.', 1)[1].lower() in current_app.config['ALLOWED_EXTENSIONS']

@account.route('/upload_avatar', methods=['POST'])
@is_login
def upload_avatar(user_id = 4):
    try:
        if 'avatar' not in request.files:
            return {"msg": "没有文件"}
        file = request.files['avatar']
        if file and allowed_file(file.filename):
            if file.filename == '':
                return {"msg": "没有文件名"}
            # 原文件名
            filename = secure_filename(file.filename)
            # 新的随机文件名
            filename = "avatar_" + str(user_id) + '_' + generate(size=18) + '.' + filename.split(".")[-1]
            # 文件名存储
            SQLHelper().update('''update user set avatar =  %s where id = %s''', (filename, user_id))
            # 文件存储
            file.save(os.path.join(current_app.config['UPLOAD_FOLDER'], filename))
            return ""
        return {"msg": "不允许上传"}
    except:
        return {"msg": "文件上传失败"}

@account.route('/get_avatar', methods=['GET'])
@is_login
def get_avatar(user_id = 4):
    try:
        user = SQLHelper().fetch_one('''select avatar from user where id = %s''', (user_id))
        avatar_url = url_for('static', filename = 'avatar/' + user.get('avatar'))
        return {'msg': '访问成功', 'avatar_url': avatar_url}
    except:
        return {"msg": "没有上传图片"}





