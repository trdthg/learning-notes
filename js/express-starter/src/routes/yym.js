const express = require('express');
const router = express.Router();

/* GET users listing. */
router.get('/', function(req, res, next) {
  res.send('respond with a resource');
});

router.get("/automap", function (req, res, next) {
    res.render("yym_automap", { title: "Tencent CloudBase + Express" })
});


module.exports = router;
