# mandelbrot
A server for generating and hosting mandelbrot set tiles

![a picture of the mandelbrot set](screenshots/screenshot.png)

## Host it yourself
Just run the mandelbrot server with docker:  
```docker run -d --name mandelbrot -v ./tiles:/tiles -p 80:80 ghcr.io/adridevelopsthings/mandelbrot:main```  
or build it yourself with ``cargo build --release``

## How to use?
Just open the url of the mandelbrot in your webbrowser.

## Environment variables
- `TILES_PATH`: Configure where the tiles should be saved. (*tiles* is the default value, */tiles* is the default value in docker)
- `PIXELS_PER_TILE`: Configure how many pixels should be displayed per row/column per tile (*255* is the default value)
- `LISTEN_ADDRESS`: Configure where the mandelbrot server should listen (*127.0.0.1:8000* is the default value, *0.0.0.0:80* is the default value in docker)
- `MAX_PREGENERATE_ZOOM_LEVEL`: Configure the max zoom level that should be pregenerated
- `MAX_AUTO_DELETE_PROTECTED_ZOOM_LEVEL`: Configure the max zoom level that wil be protected from auto deletion