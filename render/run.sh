#!/bin/bash

cargo run plasma
ffmpeg -y -i output-%03d.ppm -r 60 output.mp4