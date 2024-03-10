Docker images on `linux/amd64` with `alpine` for [wouterken/html2adf](https://github.com/wouterken/htmltoadf), instead of `linux/arm64` with `debian:buster-slim`.

**Usage**

```bash
$ echo "<h1>Hello world<p>Test</p></h1>" | docker run --rm -i aneroid/html2adf:0.1.7-alpine
{"version":1,"type":"doc","content":[{"type":"heading","attrs":{"level":1},"content":[{"type":"text","text":"Hello world"},{"type":"text","text":"Test"}]}]}
```
