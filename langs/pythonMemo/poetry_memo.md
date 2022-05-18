# install error

* used python: 3.9
* required python via numpy: >=3.7,<3.10

```diff
[tool.poetry.dependencies]
- python = "^3.9"
+ python = ">=3.7,<3.10"
```

# pyright

```toml
[tool.pyright]
include = ["src"]
exclude = ["**/node_modules", "**/__pycache__", "src/typestubs" ]
executionEnvironments = [ {root = "src"} ]
#stubPath = "src/stubs"
venvPath = "."
venv = ".venv"
```
