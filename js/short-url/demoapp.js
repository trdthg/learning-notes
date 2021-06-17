// import express, { response } from "express";
const express = require("express") 

const app = express();
app.use(express.json()) // 解析body
// app.use(express.urlencoded()) // 解析传统表单
app.use(function (req, res, next) {
  res.header("Access-Control-Allow-Origin", "*");
  res.header("Access-Control-Allow-Headers", "X-Requested-With");
  res.header("Access-Control-Allow-Methods", "PUT,POST,GET,DELETE,OPTIONS");
  next();
});
app.all('/*', function(req, res, next) {
res.header("Access-Control-Allow-Origin", "*");
res.header('Access-Control-Allow-Methods', 'PUT, GET, POST, DELETE, OPTIONS');
res.header("Access-Control-Allow-Headers", "X-Requested-With");
res.header('Access-Control-Allow-Headers', 'Content-Type');
next();

});

app.post("/create", (request, response) => {
  // 拿参数request.query
  const {url, slug} = request.body  // slug指地址别名
  response.send({url, slug, data: Date.now()})
})


module.exports = app;