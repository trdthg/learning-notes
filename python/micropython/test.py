# print("hello")
# def sss(): 
#     print(ss)

from machine import Pin,PWM
pin1=PWM(Pin(5),freq=1000)
pin2=PWM(Pin(12),freq=1000) 
pin1.duty(1000)
pin2.duty(0) 