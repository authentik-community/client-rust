.SHELLFLAGS += -x -e
PWD = $(shell pwd)
UID = $(shell id -u)
GID = $(shell id -g)

all: clean build

clean:
	rm -rf src/ docs/

build:
ifndef ref
	$(error Usage: make build ref=main version=20xx.xx.xx)
endif
ifndef version
	$(error Usage: make build ref=main version=20xx.xx.xx)
endif
	mv schema.yml schema-old.yml
	wget -O schema.yml "https://raw.githubusercontent.com/goauthentik/authentik/$(ref)/schema.yml"
	docker compose -f scripts/docker-compose.yml run --user "${UID}:${GID}" diff \
		--markdown \
		/local/diff.test \
		/local/schema-old.yml \
		/local/schema.yml
	rm schema-old.yml
	docker compose -f scripts/docker-compose.yml run --user "${UID}:${GID}" gen \
		generate \
		-i /local/schema.yml \
		-g rust \
		-o local \
		-c /local/config.yaml \
		--additional-properties=packageVersion=$(version)
	rm -f .travis.yml git_push.sh
	cargo fmt
	mv diff.test /tmp/diff.test
	echo "Update API Client\n\n" > diff.test
	cat /tmp/diff.test >> diff.test
