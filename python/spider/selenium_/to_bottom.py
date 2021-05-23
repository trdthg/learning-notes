from selenium import webdriver
import time
driver = webdriver.Chrome()
driver.get("http://www.zhihu.com/explore")
driver.execute_script('window.scrollTo(0,document.body.scrollHeight)')
time.sleep(1)
html = driver.execute_script("return document.documentElement.outerHTML")
html = driver.page_source