import network
import webrepl 
import time 
from time import sleep  
import urequests

# 某几个默认wifi
SSIDs = [("602", "4602yyds"), ("", "")]  

def do_connect():     
    import network     
    wlan = network.WLAN(network.STA_IF)     
    wlan.active(True)     
    start = time.time()     
    while not wlan.isconnected():         
        print('connecting to network...')         
        wlan.connect(SSIDs[0][0], SSIDs[0][1])         
        sleep(1)         
        if time.time() - start > 20:             
            print("connect timeout!")             
            break     
    if wlan.isconnected():         
        print("successfully connected")         
        print('network config:', wlan.ifconfig())  

def get_weather():
    res = urequests.get("www.baidu.com")
    print(res)

if __name__ == "__main__":
    do_connect()  
    get_weather()
