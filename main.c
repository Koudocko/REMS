#include <stdio.h>
#include <fcntl.h>
#include <termios.h>
#include <unistd.h>
#include <string.h>

int main() {
    int fd = open("/dev/ttyACM1", O_RDWR | O_NOCTTY | O_NONBLOCK);
    FILE* log_fd = fopen("/rems/readings/log1.txt", "w");
    if (fd == -1 || log_fd == NULL) {
        perror("Error opening serial port");
        return 1;
    }

    struct termios options;
    tcgetattr(fd, &options);
    cfsetispeed(&options, B9600);
    cfsetospeed(&options, B9600);
    options.c_cflag |= (CLOCAL | CREAD);
    options.c_cflag &= ~PARENB;
    options.c_cflag &= ~CSTOPB;
    options.c_cflag &= ~CSIZE;
    options.c_cflag |= CS8;
    tcsetattr(fd, TCSANOW, &options);

    char buffer[4096];
    while (1){
        int bufferSize = 0;
        char* mark = NULL;

        while ((bufferSize = read(fd, buffer, sizeof(buffer)), 
            mark = (char*)memchr(buffer, '{', bufferSize)) == NULL){
            usleep(1000);
        }

        memmove(buffer, mark, &buffer[bufferSize] - mark);

        if ((mark = (char*)memchr(buffer, '}', bufferSize)) == NULL){
            int readSize = 0;
            while ((readSize = read(fd, buffer + bufferSize, sizeof(buffer) - bufferSize), 
                mark = (char*)memchr(buffer + bufferSize, '}', readSize)) == NULL){
                bufferSize += readSize;
                usleep(1000);
            }
        }

        bufferSize = (mark - buffer) + 1; 
        buffer[bufferSize] = '\0';

        printf("%s\n", buffer);
        fprintf(log_fd, "%s\n", buffer);
        rewind(log_fd);
        fflush(log_fd);
    }

    fclose(log_fd);
    close(fd);
    return 0;
}
