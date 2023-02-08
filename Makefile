clean:
	rm -rf data/
	mkdir data
test: clean
	cargo test
test-http: clean
	resty http_endpoint_tests.json
