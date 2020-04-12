# res
utility for measuring and displaying resources used by a program


# Usage

```sh
res python -c "import functools; functools.reduce(lambda a,b : str(a)+str(b), range(1000000));"
0.00 CPU% 107.67 RSS 196KB VIRT 4GB
0.00 CPU% 99.65 RSS 43MB VIRT 4GB
0.00 CPU% 99.77 RSS 48MB VIRT 4GB
0.00 CPU% 99.85 RSS 51MB VIRT 4GB
0.00 CPU% 99.82 RSS 54MB VIRT 4GB
0.00 CPU% 99.89 RSS 56MB VIRT 4GB
0.00 CPU% 99.89 RSS 58MB VIRT 4GB
0.00 CPU% 99.85 RSS 60MB VIRT 4GB
```
