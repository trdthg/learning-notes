FROM alpine:latest

RUN echo "PATH=$PATH:~/.npm-packages" > ~/.bashrc \
	&& sed -i 's/dl-cdn.alpinelinux.org/mirrors.ustc.edu.cn/g' /etc/apk/repositories \
\
	&& apk add --no-cache \
	git \
	bash \
	nodejs \
	yarn \
	&& cd ~ \
	&& git clone https://github.com/xhebox/ioclub.git

RUN echo "$http_proxy" \
	&& echo "$https_proxy"

RUN cd ~/ioclub \
	&& git checkout nextgen \
	&& rm ./.yarnrc \
	&& yarn build-submodule \ 
	&& yarn install \
	&& yarn build-pkgs \
	&& yarn dev

USER admin
EXPOSE 3000

