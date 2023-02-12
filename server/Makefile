clean:
	rm -rf data/
	mkdir data

test: clean
	cargo test

test-http: test-http-create test-http-read test-http-update test-http-delete test-http-exists

test-http-create: clean
	resty endpoint-tests/http_create.json

test-http-read: clean
	resty endpoint-tests/http_read.json

test-http-update: clean
	resty endpoint-tests/http_update.json

test-http-delete: clean
	resty endpoint-tests/http_delete.json

test-http-exists: clean
	resty endpoint-tests/http_exists.json
