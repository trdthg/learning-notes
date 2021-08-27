import pyb from pyb 
import UART from pyb 
import Pin 
y1 = Pin('Y1', Pin.OUT_PP) 
y2 = Pin('Y2', Pin.OUT_PP) 
y3 = Pin('Y3', Pin.OUT_PP) 
y4 = Pin('Y4', Pin.OUT_PP) 
x1 = Pin('X1', Pin.IN) 
x2 = Pin('X2', Pin.IN) 
x3 = Pin('X3', Pin.IN) 
x4 = Pin('X4', Pin.IN) 
def go(): 
    y1.high() 
    y4.high() 
    y2.low() 
    y3.low() 

def left(): 
    y2.high() 
    y4.high() 
    y1.low() 
    y3.low() 

def right(): 
    y1.high() 
    y3.high() 
    y2.low() 
    y4.low() 
    
def back(): 
    y2.high() 
    y3.high() 
    y1.low() 
    y4.low() 

#检测到返回1 
print(x1.value()) 
while True: 
    if(x1.value()==1):#检测到物体返回1。 
        left() 
        pyb.delay(50) 
    if(x4.value()==1):#检测到物体返回1。 
        right() 
        pyb.delay(50) 
    if(x2.value()&x3.value()==1): 
        go() 
        pyb.delay(50)