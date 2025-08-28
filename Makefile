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
	docker run --rm \
		-v "${PWD}:/local" \
		--user "${UID}:${GID}" \
		docker.io/openapitools/openapi-diff:2.1.2 \
		--markdown /local/diff.test \
		/local/schema-old.yml /local/schema.yml || echo > diff.test
	rm schema-old.yml
	docker run --rm \
		-v "${PWD}:/local" \
		--user "${UID}:${GID}" \
		docker.io/openapitools/openapi-generator-cli:v7.15.0 \
		generate \
		-i /local/schema.yml \
		-g rust \
		-o /local \
		-c /local/config.yaml
	rm -f .travis.yml git_push.sh
	cargo fmt
	mv diff.test /tmp/diff.test
	echo "Update API Client\n\n" > diff.test
	cat /tmp/diff.test >> diff.test
