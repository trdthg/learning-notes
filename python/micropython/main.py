import webrepl
import network
import time
from time import sleep


def do_connect():
    SSIDs = [("PHONE_trdthg_LGV35", "hj689753"), ("602", "4602yyds"), ("408", "504504504")]
    wlan = network.WLAN(network.STA_IF)
    wlan.active(True)
    start = time.time()
    while not wlan.isconnected():
        print('connecting to network...')
        wlan.connect(SSIDs[0][0], SSIDs[0][1])
        sleep(1)
        if time.time() - start > 3:
            print("connect timeout!")
            break
    if wlan.isconnected():
        print("successfully connected")
        print("you can use the local ip to connect to esp8266 and upload files")
        print('network config:', wlan.ifconfig())





# def get_weather():
#     import urequests
#     res = urequests.get("www.baidu.com")
#     print(res)
#     pass
#     print(sss)

class DCMotor:

  def __init__(self, pin1, pin2, enable_pin, min_duty=750, max_duty=1023):
    self.pin1 = pin1
    self.pin2= pin2
    self.enable_pin = enable_pin
    self.min_duty = min_duty
    self.max_duty = max_duty

  def forward(self, speed):
    self.speed = speed
    self.enable_pin.duty(self.duty_cycle(self.speed))
    self.pin1.value(1)
    self.pin2.value(0)

  def backwards(self, speed):
    self.speed = speed
    self.enable_pin.duty(self.duty_cycle(self.speed))
    self.pin1.value(0)
    self.pin2.value(1)

  def stop(self):
    self.enable_pin.duty(0)
    self.pin1.value(0)
    self.pin2.value(0)

  def duty_cycle(self, speed):
    if self.speed <= 0 or self.speed > 100:
      duty_cycle = 0
    else:
      duty_cycle = int (self.min_duty + (self.max_duty - self.min_duty)*((self.speed - 1)/(100-1)))
    return duty_cycle

def run():
    from machine import Pin,PWM
    pin1=PWM(Pin(5),freq=1000)
    pin2=PWM(Pin(12),freq=1000) 
    def car_forward():
        pin1.duty(1000)  #左前
        pin2.duty(0)
    i = 0
    while True:
        print(i, end=" ")
        car_forward()
        print("success")
        i+=1
        if i > 10000:
            break

if __name__ == "__main__":
    print("hello")
    do_connect()
    try:
        run()
        print("run success")
    except:
        print("run false")

