.SHELLFLAGS += -x -e
PWD = $(shell pwd)
UID = $(shell id -u)
GID = $(shell id -g)

all: clean build

clean:
	rm -rf src/ docs/

build:
ifndef version
	$(error Usage: make build version=version/20.xx.xx.xx)
endif
	mv schema.yml schema-old.yml
	wget -O schema.yml "https://raw.githubusercontent.com/goauthentik/authentik/$(version)/schema.yml"
	docker compose -f scripts/docker-compose.yml run --user "${UID}:${GID}" diff
	rm schema-old.yml
	docker compose -f scripts/docker-compose.yml run --user "${UID}:${GID}" gen
	rm -f .travis.yml git_push.sh
	cargo fmt
	mv diff.test /tmp/diff.test
	echo "Update API Client\n\n" > diff.test
	cat /tmp/diff.test >> diff.test
