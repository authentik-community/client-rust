.SHELLFLAGS += -x -e
PWD = $(shell pwd)
UID = $(shell id -u)
GID = $(shell id -g)

build:
ifndef version
	$(error Usage: make build version=20xx.xx.xx)
endif
	rm -rf src/ docs/
	docker compose -f compose.yml run --rm --user "${UID}:${GID}" gen \
		generate \
		-i /local/schema.yml \
		-g rust \
		-o local \
		-c /local/config.yaml \
		--additional-properties=packageVersion=$(version)
	rm -f .travis.yml git_push.sh
	cargo fmt
