DOCKER_NAME ?= puzzleos
.PHONY: docker build_docker
	
docker:
	docker run -itd -p 8888:8888 -v ${PWD}:/mnt -w /mnt --name puzzleos ${DOCKER_NAME} bash

build_docker: 
	docker build -t ${DOCKER_NAME} .

fmt:
	cd easy-fs; cargo fmt; cd ../easy-fs-fuse cargo fmt; cd ../os ; cargo fmt; cd ../user; cargo fmt; cd ..
