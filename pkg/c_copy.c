#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <stdio.h>
#include <unistd.h>
#include <string.h>
#include <stdlib.h>
int main(int argc, char **argv)
{
	int fdSrc;
	int fdDes;
	
	char *readBuf = NULL;

	if(argc != 3){
		printf("pararm error\n");
		exit(-1);
	}

	fdSrc = open(argv[1],O_RDWR);
	int size = lseek(fdSrc, 0,SEEK_END);
	lseek(fdSrc,0,SEEK_SET);
	
	readBuf = (char *)malloc(size*sizeof(char)+8);
	int n_read = read(fdSrc,readBuf,1024);

	fdDes = open(argv[2],O_RDWR|O_CREAT,0600);
	
	int n_write = write(fdDes, readBuf,strlen(readBuf));

	
	
	close(fdSrc);
	close(fdDes);

	return 0;
}
