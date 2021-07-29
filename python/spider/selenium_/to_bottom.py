# edge-driver链接 https://developer.microsoft.com/en-us/microsoft-edge/tools/webdriver/#downloads

# driver = webdriver.Edge()
# driver.get("http://www.zhihu.com/explore")
# driver.execute_script('window.scrollTo(0,document.body.scrollHeight)')
# time.sleep(1)
# html = driver.execute_script("return document.documentElement.outerHTML")
# html = driver.page_source

from selenium import webdriver
from selenium.webdriver.chrome.service import Service
from time import sleep

driver_url = "C:\\Users\\YMZ\\AppData\\Local\\Microsoft\\Edge SxS\\Application\\msedgedriver.exe"
service = Service(driver_url)
service.start()
dr = webdriver.Remote(service.service_url)

dr.get('https://www.baidu.com/')

dr.find_element('id', 'kw').send_keys('博客园 韩志超')
dr.find_element('id', 'su').click()
sleep(3)

dr.quit()