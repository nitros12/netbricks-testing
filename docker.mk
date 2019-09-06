SANDBOX ?= williamofockham/sandbox:nightly-2019-07-03

MOUNTS = -v /lib/modules:/lib/modules \
		 -v /usr/src:/usr/src \
		 -v /dev/hugepages:/dev/hugepages \
		 -v /mnt/huge:/mnt/huge \
		 -v /dev/uio0:/dev/uio0 \

.PHONY: pull-sandbox run-test

pull-sandbox:
	@docker pull $(SANDBOX)

build-test-sandbox:
	@docker build -t zizek-test/test .

run-test: pull-sandbox build-test-sandbox
	@docker run -it --rm --privileged --network=host \
		-w /opt \
		-e HIST_FILE=/root/.bash_history \
		-v $(HOME)/.bash_history:/root/.bash_history \
		$(MOUNTS) \
		zizek-test/test /bin/bash -c "cd /opt/netbricks_test && ./run.sh"
