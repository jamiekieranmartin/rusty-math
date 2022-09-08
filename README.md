# rusty-math

a bit of rusty math served via REST API

> this is as much brain power as I want to give ._.

## Area of a Circle

```sh
curl --location --request POST 'http://localhost:8000/circle/area' \
--header 'Content-Type: application/json' \
--data-raw '{
    "radius": 2
}'
```

## Area of a Triangle

```sh
curl --location --request POST 'http://localhost:8000/triangle/area' \
--header 'Content-Type: application/json' \
--data-raw '{
    "a": {
        "x": -2,
        "y": 3
    },
    "b": {
        "x": -3,
        "y": -1
    },
    "c": {
        "x": 3,
        "y": -2
    }
}'
```

## Area of a Rectangle

```sh
curl --location --request POST 'http://localhost:8000/rectangle/area' \
--header 'Content-Type: application/json' \
--data-raw '{
    "top_left": {
        "x": 0,
        "y": 0
    },
    "bottom_right": {
        "x": 2,
        "y": 2
    }
}'
```
