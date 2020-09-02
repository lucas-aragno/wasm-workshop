## WASM Examples

First clone [this](https://github.com/emscripten-core/emsdk) repository and build the docker image (you need to have docker previously installed, it will save you time on the long run).

```bash
  docker build -f emsdk/docker/Dockerfile . -t emcc
```

Now you can run the emcc image wherever you want by doing:

```bash
docker run -it emcc bash 
```

Now go through the `README.md` of each demo folder to see how to run every example.