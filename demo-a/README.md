# Demo A

Run it: 

```
docker run -v ${PWD}/examples:/src/examples -it emcc bash
```

the `-v` option will create a volume on the examples folder so we can share it between our host and container machines.

Once there you can run:

```bash
emcc examples/main.c -o examples/main.js
```

or the `HTML` version

```
emcc examples/main.c -o examples/main.html
```