from flask import Flask
from flask import url_for
from flask import request

app = Flask(__name__)

# 1. part test

# @app.route('/')
# def index():
#     return 'index'

@app.route('/user/<username>')
def profile(username):
    return f'{username}\'s profile'


# get json request data

# @app.route('/login', methods=['POST', 'GET'])
# def login():
#     error = None
#     if request.method == 'POST':
#         if valid_login(request.form['username'],
#                        request.form['password']):
#             return log_the_user_in(request.form['username'])
#         else:
#             error = 'Invalid username/password'
#     elif request.method == 'GET':
#         req_data = request.get_json()
#     return render_template('login.html', error=error)

# redirect and abort
from flask import abort, redirect, url_for

@app.route('/')
def index():
    return redirect(url_for('login'))

# @app.route('/login')
# def login():
#     abort(401)
#     this_is_never_executed()

# personlize error page
@app.errorhandler(404)
def page_not_found(error):
    return render_template('page_not_found.html'), 404

# session
from flask import session

# Set the secret key to some random bytes. Keep this really secret!
app.secret_key = b'_5#y2L"F4Q8z\n\xec]/'

# @app.route('/')
# def index():
#     if 'username' in session:
#         return f'Logged in as {session["username"]}'
#     return 'You are not logged in'

# @app.route('/login', methods=['GET', 'POST'])
# def login():
#     if request.method == 'POST':
#         session['username'] = request.form['username']
#         return redirect(url_for('index'))
#     return '''
#         <form method="post">
#             <p><input type=text name=username>
#             <p><input type=submit value=Login>
#         </form>
#     '''

@app.route('/logout')
def logout():
    # remove the username from the session if it's there
    session.pop('username', None)
    return redirect(url_for('index'))

if __name__ == "__main__":
    with app.test_request_context():
        print(url_for('index'))
        print(url_for('profile', username='John Doe'))
        print(url_for('static', filename='style.css'))
    app.run(host='localhost', port=8080)
    
