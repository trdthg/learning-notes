const app = require("./demoapp2")
app.listen(3080, () => {
    console.log('server is ready > https://localhost:3080')
})

// import { listen } from "./app"
// listen(3080, () => {
//     console.log('server is ready > https://localhost:3080')
// })