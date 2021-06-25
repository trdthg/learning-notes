const express = require("express")
const router = express.Router();

const {nanoid} = require('nanoid')

const cloudbase = require('@cloudbase/node-sdk')
const cloudbase_app = cloudbase.init({})
var db = cloudbase_app.database();

/* GET home page. */
router.get("/", function (req, res, next) {
  res.render("index", { title: "Tencent CloudBase + Express" })
});

router.get("/yym_automap", function (req, res, next) {
  res.render("yym_automap", { title: "Tencent CloudBase + Express" })
});

router.get("/getNanoid", (req, res, next) => {
  try {
    let newid = nanoid(6)
    res.json({status: 200, message: "成功访问getNanoid, nanoid是随机生成的6位字符串", nanoid: newid})
  } catch (error) {
  }
  res.json({status: 0, message: "失败访问getNanoid", nanoid: 0})

})

router.post("/create", async (req, res, next) => {
  const {url, slug = nanoid(6)} = req.body
  // 逻辑判断...
  await db.collection('links').add({slug, url}) // add方法是异步执行，需要await async
  res.json({ link: 'https://localhost:3080/' + slug }) 
})

// 登录
router.post("/login", async (req, res, next) => {
  const {username, password} = req.body
  // 逻辑判断...
  let a = await db.collection('links')
  .where({
    username: username
  }).get()
   // add方法是异步执行，需要await async
  if (a.data[0].password == password) {
    res.json({ status: 0, msg: "登陆成功", token: "aswefwesfwkjadheukwaebksb23476297siagduked73qoh=="}) 
  } else {
    res.json({status: 1, msg: "没有该用户"})
  }
})

// 注册
router.post("/register", async (req, res, next) => {
  const {username, password} = req.body
  // 逻辑判断...
  await db.collection('links').add({username, password}) // add方法是异步执行，需要await async
  res.json({ status: 0, msg: "新建用户成功", token: "aswefwesfwkjadheukwaebksb23476297siagduked73qoh=="}) 
})


module.exports = router;
