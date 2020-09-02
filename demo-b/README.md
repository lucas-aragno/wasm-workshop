# Demo B

Run it: 

```
docker run -v ${PWD}/examples:/src/examples -it emcc bash
```

the `-v` option will create a volume on the examples folder so we can share it between our host and container machines.

Once there you can run:

```bash
emcc examples/fib.c -o examples/fib.js -s WASM=1 -s EXPORTED_FUNCTIONS='["_fib"]' -s EXTRA_EXPORTED_RUNTIME_METHODS='["cwrap"]'
```

Then you can run a webserver

```
python -m SimpleHTTPServer