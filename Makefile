test:
	pytest && make doctest

lint:
	flake8 && mypy .

fmt:
	black .
