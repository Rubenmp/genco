# Contributing

Inside the project main folder, add a file in the root ".git/hook/pre-commit" with this content:

```bash
#!/bin/sh

cargo fix --allow-dirty --allow-staged > /dev/null 2>&1
cargo fmt > /dev/null 2>&1
git add .
```
