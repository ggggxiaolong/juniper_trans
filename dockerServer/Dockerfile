FROM ubuntu:18.04
#声明作者
MAINTAINER TAN

#在image中创建文件夹
RUN mkdir -p /home/Service  && \
    apt-get update && \
    apt-get -y install libsqlite3-0 libssl1.1 libssl-dev ca-certificates
#将该文件夹作为工作目录
WORKDIR /home/Service

# 将node工程下所有文件拷贝到Image下的文件夹中
COPY . /home/Service

EXPOSE 4000
#执行npm start命令，启动Node工程
CMD [ "./juniper" ]
