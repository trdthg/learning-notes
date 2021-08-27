# from dcmotor import DCMotor
from machine import Pin, PWM
from time import sleep

frequency = 15000

pin1 = Pin(12, Pin.OUT)
pin2 = Pin(14, Pin.OUT)
enable = PWM(Pin(13), frequency)

dc_motor = DCMotor(pin1, pin2, enable)

dc_motor.forward(50)
sleep(2)
dc_motor.stop()
sleep(1)
dc_motor.backwards(100)
sleep(2)
dc_motor.forward(10)
sleep(5)
dc_motor.stop()

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