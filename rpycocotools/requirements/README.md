### Requirements generation

The requirements can be re-generated with the following commands
```
pip-compile --output-file=requirements/requirements.txt --resolver=backtracking pyproject.toml --generate-hashes
pip-compile --extra=test --output-file=requirements/requirements-test.txt --resolver=backtracking pyproject.toml --generate-hashes
pip-compile --extra=dev --output-file=requirements/requirements-dev.txt --resolver=backtracking pyproject.toml --generate-hashes
pip-compile --extra=sam --output-file=requirements/requirements-sam.txt --resolver=backtracking pyproject.toml --generate-hashes
pip-compile --extra=flake8 --output-file=requirements/requirements-flake8.txt --resolver=backtracking pyproject.toml --generate-hashes
```