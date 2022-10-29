test:
	pytest

lint:
	flake8 && mypy .

fmt:
	black .
