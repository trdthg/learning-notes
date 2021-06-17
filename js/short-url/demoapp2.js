const express = require("express") 
const { init } = require("@cloudbase/node-sdk")
const {nanoid} = require('nanoid')

const tcb = init({ env:'hello-cloudbase-5gsjfurqb730a0c8', secretId:'AKIDdBWeYDdqtpX0GH2ZruhEj146vUotjKju', secretKey:'ziCQP8bpnWzC9qHTQ7ZliRvAAuQksmRr'})

const db = tcb.database();

const app = express();

app.use(express.json())

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

app.post("/create", async (request, response) => {
  console.log(1);
  const {url, slug = nanoid(6)} = request.body
  // 逻辑判断...
  await db.collection('links').add({slug, url}) // add方法是异步执行，需要await async
  response.send({ link: 'https://localhost:3080/' + slug }) 
})

app.get('/:slug', async (req, res) => {
    console.log(req.params)
    const { slug } = req.params
    const { data:[link]} = await db.collection('links').where({ slug }).get()  // get返回的是集合，data解构出第一个赋值link
    // 逻辑判断...
    
    res.send(link)
})

// 与上一个冲突
// app.get('/get', (req, res) => {
//   res.send({data: "www"})
// })

module.exports = app;