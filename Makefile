# .PHONY: test

test:
	cd ./sparse_matrix_rust_to_python && maturin develop
	python ./scripts/test.py