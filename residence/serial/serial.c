#include <stdio.h>
#include <fcntl.h>
#include <termios.h>
#include <unistd.h>
#include <string.h>

int main() {
    // Attach to the serial device
    int fd = open("/dev/ttyACM0", O_RDWR | O_NOCTTY | O_NONBLOCK);
    if (fd == -1) {
        perror("Error opening serial port");
        return 1;
    }

    // Set various flags to match arduino's serial communication protocol
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

    // Reset the Arduino to sync the data stream (the IDE does this by default, but cli does not)
    options.c_cflag &= ~HUPCL; 
    tcsetattr(fd, TCSANOW, &options);
    usleep(2500000);  
    options.c_cflag |= HUPCL;  
    tcsetattr(fd, TCSANOW, &options);
    usleep(2500000);  

    // Extract data between two curly braces {} continuously and write it to a localized file
    char buffer[4096];
    while (1){
        int bufferSize = 0;
        char* mark = NULL;

        // Find starting point
        while ((bufferSize = read(fd, buffer, sizeof(buffer)), 
            mark = (char*)memchr(buffer, '{', bufferSize)) == NULL){
            usleep(1000);
        }

        memmove(buffer, mark, &buffer[bufferSize] - mark);

        // Find end point
        if ((mark = (char*)memchr(buffer, '}', bufferSize)) == NULL){
            int readSize = 0;
            while ((readSize = read(fd, buffer + bufferSize, sizeof(buffer) - bufferSize), 
                mark = (char*)memchr(buffer + bufferSize, '}', readSize)) == NULL){
                bufferSize += readSize;
                usleep(1000);
            }
        }

        // Mark end of the JSON package
        bufferSize = (mark - buffer) + 1; 
        buffer[bufferSize] = '\0';

        printf("%s\n", buffer);

        // Write the extracted JSON to the file
        FILE* log_fd = fopen("/rems/readings/residence.json", "w");
        if (log_fd != NULL){
            fprintf(log_fd, "%s\n", buffer);
            fflush(log_fd);
            fclose(log_fd);
        }

        usleep(1000);
    }

    close(fd);
    return 0;
}
