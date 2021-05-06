from flask import Flask

app = Flask(__name__)

@app.after_request
def cors(environ):
    environ.headers['Access-Control-Allow-Origin']='*'
    environ.headers['Access-Control-Allow-Method']='*'
    environ.headers['Access-Control-Allow-Headers']='x-requested-with,content-type'
    return environ

@app.route('/python/hello', methods=['GET','POST'])
def hello_world():
    print("s")
    return 'hello world'

if __name__ == '__main__':
    app.run(host='localhost',port=9999)