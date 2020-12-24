import multiprocessing as mp

def main():
    multicore()
    # job(100000000)
def job(x):
    flag = 0
    for j in range(2, int(x**0.5)+1):
        if x % j == 0:
            flag = 1
            break
    if flag == 0:
        return x
    else:
        return 0
    # print (m)

def multicore():
    pool = mp.Pool()
    result = pool.map(job,range(1,100000000))
    # result = job(100)
    print(result)

if __name__ == '__main__':
    main()