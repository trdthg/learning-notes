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

router.get("/a", function (req, res, next) {
  res.json({status: 200, message: '本地上传'})
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


module.exports = router;
