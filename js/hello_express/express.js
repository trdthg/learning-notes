const express = require("express")

app = express()

app.get('/', function(req, res) {
    res.send("hello express")
})

app.listen(3000)