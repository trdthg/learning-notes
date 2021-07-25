import pyautogui

import time


try:
    while True:

        x,y=pyautogui.position()
        postr='x: '+str(x)+' y: '+str(y)
        print('\r'+postr,end='')
        print('\r',end='',flush=True)



except KeyboardInterrupt:
    print('\n')