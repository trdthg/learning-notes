import matplotlib.pyplot as plt
import numpy as np
#img data
a = np.array([0.1,0.2,0.3,0.4,0.5,0.6,0.7,0.8,0.9]).reshape(3,3)

plt.imshow(a, interpolation='nearest', cmap='bone', origin='upper')

plt.colorbar(shrink=0.9)

plt.show()