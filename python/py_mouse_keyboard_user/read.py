import requests
import json

from selenium import webdriver
from selenium.webdriver.chrome.service import Service
from selenium.webdriver.common.by import By
from time import sleep

headers = {
    'Host': 'c.y.qq.com',
    'Referer': 'http://c.y.qq.com/',
    'User-Agent': 'Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.107 '
    'Safari/537.36 '
}


def find_song(word):
        """
    查找歌曲
    :param word: 歌曲名
    :return: 返回歌曲mid
    """
        get_url = 'https://c.y.qq.com/soso/fcgi-bin/client_search_cp?&t=0&aggr=1&cr=1&catZhida=1&lossless=0&flag_qc=0&p=1&n' \
        '=20&w=' + word
        try:
            res1 = requests.get(get_url, headers=headers)
            get_json = json.loads(res1.text.strip('callback()[]'))
            jsons = get_json['data']['song']['list']
            songmid = []
            media_mid = []
            song_singer = []
            i = 1
            for song in jsons:
                print(i, ':' + song['songname'], '---', song['singer'][0]['name'])
                songmid.append(song['songmid'])
                media_mid.append(song['media_mid'])
                song_singer.append(song['singer'][0]['name'])
                i = i + 1
            return songmid
        except Exception as e:
            print(f'歌曲查找有误：{e}')
            return None
 
def getweburl(mid):
    print("----------------------------------------------------------------------")
    print("page1")
    # 进入查看界面
    url = f"https://y.qq.com/n/ryqq/songDetail/{mid}"
    dr.get(url)
    sleep(2)
    # 跳转到试听界面
    kk = 0
    try: 
        dr.find_elements_by_class_name("mod_btn_green__icon_play")[0].click()
        print("page2")
        windows = dr.window_handles
        dr.switch_to.window(windows[1])
        kk = 1
        sleep(5)
    except:
        pass
    if kk == 1:
        try:
            dr.find_elements_by_class_name("upload_btns__item mod_btn_green mod_btn")[0].click()
        except:
            pass
        sleep(2)
        try:
            dr.find_elements_by_class_name("specialbutton")[0].click()
            # 跳转到外部下载页
            print("page3", )
            sleep(2)
            windows = dr.window_handles
            dr.switch_to.window(windows[2])
            kk = 2
        except:
            pass
        if kk == 2:
            # # 真正的下载页面
            # dr.find_elements_by_class_name("am-btn am-btn-default")[0].click()
            # windows = dr.window_handles(By.CSS)
            # dr.switch_to.window(windows[1])
            url = dr.find_element_by_id("j-src-btn").get_attribute("href")
            song_name = dr.find_element_by_id("j-src-btn").get_attribute("download")
            # url = dr.find_elements_by_class_name("am-btn am-btn-default")[0].get_attribute("href")
            # song_name = dr.find_elements_by_class_name("am-btn am-btn-default")[0].get_attribute("download")

            print("-----")
            print(song_name)
            print(url)
            print("-----")
            dr.close()
            dr.switch_to.window(windows[1])
            dr.close()
            dr.switch_to.window(windows[0])
            try:
                r = requests.get(url)
                with open(song_name, "wb") as code:
                    code.write(r.content)
                return 1
            except:
                pass
        else:
            dr.close()
            dr.switch_to.window(windows[0])
    return 0
    
    
def get_driver():
    driver_url = "C:\\Users\\YMZ\\AppData\\Local\\Microsoft\\Edge SxS\\Application\\msedgedriver.exe"
    user_data = "C:\\Users\\YMZ\\AppData\\Local\\Microsoft\\Edge\\User Data"
    # option = webdriver.ChromeOptions()
    # option.add_argument(f"--user-data-dir={user_data}")
    # print(option)
    service = Service(driver_url)
    service.start()
    dr = webdriver.Remote(service.service_url)

    # from selenium.webdriver.edge.options import Options
    # option = Options()
    # dr = webdriver.Edge()
    
    # from selenium.webdriver.chrome.options import Options
    # option = Options()
    # dr = webdriver.Chrome(options = option)
    return dr
    
if __name__ == "__main__":
    # dr = get_driver()
    driver_url = "C:\\Users\\YMZ\\AppData\\Local\\Microsoft\\Edge SxS\\Application\\msedgedriver.exe"
    user_data = "C:\\Users\\YMZ\\AppData\\Local\\Microsoft\\Edge\\User Data"
    service = Service(driver_url)
    service.start()
    dr = webdriver.Remote(service.service_url)

    # 模拟登录
    url = "https://y.qq.com/n/ryqq/songDetail/001yS0ZP150JAo"
    dr.get(url)
    dr.find_elements_by_class_name("top_login__link")[0].click()
    sleep(60)
    with open("C:\\Users\\YMZ\\Desktop\\nativeProject\\python\\py_mouse_keyboard_user\\names") as names:
        for row in names:
            name = row.strip()
            try:
                mids = find_song(name)
                i = 0
                for mid in mids:
                    try:
                        if getweburl(mid) == 1:
                            i += 1
                    except:
                        pass
                    if i == 2:
                        break
            except:
                pass
    dr.quit()
    