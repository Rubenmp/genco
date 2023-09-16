# Contributing

Inside the project main folder, add a file in the root ".git/hook/pre-commit" with this content:

```bash
#!/bin/sh

cargo fmt
```
