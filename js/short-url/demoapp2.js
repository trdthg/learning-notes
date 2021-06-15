const express = require("express") 
const { init } = require("@cloudbase/node-sdk")
const {nanoid} = require('nanoid')

const tcb = init({ env:'hello-cloudbase-5gsjfurqb730a0c8', secretId:'AKIDdBWeYDdqtpX0GH2ZruhEj146vUotjKju', secretKey:'ziCQP8bpnWzC9qHTQ7ZliRvAAuQksmRr'})

const db = tcb.database();

const app = express();

app.use(express.json())

app.post("/create", async (request, response) => {
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

module.exports = app;