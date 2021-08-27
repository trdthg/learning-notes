from  import Pin, Timer
#伸缩杆
rod_P1 = Pin('X9', Pin.OUT_PP)
rod_P2 = Pin('X10', Pin.OUT_PP)
#伸缩杆(rod)对应定时器
tim_rod = Timer(4, freq=100) #要实现每秒0.1ms量级
rod_go = tim_rod.channel(1, Timer.PWM, pin=rod_P1)
rod_back = tim_rod.channel(2, Timer.PWM, pin=rod_P2)
while True:
	#前进
	rod_go.pulse_width_percent(100) # =Pin.high() 
	rod_back.pulse_width_percent(0) # =Pin.low()
	pyb.delay(1000) #调时间就是调距离
	#后退
	rod_go.pulse_width_percent(0)  
	rod_back.pulse_width_percent(100) 
	pyb.delay(1000) #调时间就是调距离

