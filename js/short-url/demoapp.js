// import express, { response } from "express";
const express = require("express") 

const app = express();
app.use(express.json()) // 解析body
// app.use(express.urlencoded()) // 解析传统表单


app.post("/create", (request, response) => {
  // 拿参数request.query
  const {url, slug} = request.body  // slug指地址别名
  response.send({url, slug, data: Date.now()})
})


module.exports = app;