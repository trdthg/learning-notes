from flask import Flask, request
app = Flask("hello_server")

@app.route('/hello', methods=['get'])
def hello_world():
    name = request.args.get("name", default="world")
    return "<h1>" + "hello " + name + "</h1>"

app.run(host="0.0.0.0", port=5000)
