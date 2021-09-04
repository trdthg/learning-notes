# for i in range(10000000000000000000000):
#     print(f'{i}', end="\r")

import requests
url = 'https://api.github.com/repos/psf/requests'
r = requests.get(url)
print(r.status_code)

a = {"aaa": 1, "bbb": 2}