const express = require('express');
const router = express.Router();

/* GET users listing. */
router.get('/', function(req, res, next) {
  res.render("yym/index", { title: "Tencent CloudBase + Express" })
});

router.get('/Cloud-bfs-canvas', function(req, res, next) {
  res.render("yym/Cloud-bfs-canvas", { title: "Tencent CloudBase + Express" })
});

router.get('/Cloud-bfs', function(req, res, next) {
  res.render("yym/Cloud-bfs", { title: "Tencent CloudBase + Express" })
});


router.get('/Cloud-dfs-Animation-A-canvas', function(req, res, next) {
  res.render("yym/Cloud-dfs-Animation-A-canvas", { title: "Tencent CloudBase + Express" })
});

router.get('/Cloud-dfs-Animation-A', function(req, res, next) {
  res.render("yym/Cloud-dfs-Animation-A", { title: "Tencent CloudBase + Express" })
});

router.get('/Cloud-dfs-Animation-B-canvas', function(req, res, next) {
  res.render("yym/Cloud-dfs-Animation-B-canvas", { title: "Tencent CloudBase + Express" })
});

router.get('/Cloud-dfs-Animation-B', function(req, res, next) {
  res.render("yym/Cloud-dfs-Animation-B", { title: "Tencent CloudBase + Express" })
});

module.exports = router;
