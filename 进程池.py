import multiprocessing as mp
def main():
    multicore()

def job(x):
    flag = 0
    for j in range(2, int(x**0.5)+1):
        if x % j == 0:
            flag = 1
            break
    if flag == 0:
        print(x)
        return x
    else:
        return 0

def multicore():
    pool = mp.Pool()
    pool.map(job,range(1,10000))
   

    # print('多核遍历中')
    # pool = mp.Pool(16)
    # i = 0
    # for arrival in range(10000):
    #     i += 1
    #     # print(i)
    #     pool.apply_async(job, (arrival,))   #维持执行的进程总数为processes，当一个进程执行完毕后会添加新的进程进去
    # pool.close()
    # pool.join()   #调用join之前，先调用close函数，否则会出错。执行完close后不会有新的进程加入到pool,join函数等待所有子进程结束
    # print ("Sub-process(es) done.")

if __name__ == '__main__':
    main()