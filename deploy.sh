#!/bin/bash

cargo skyline build --release
cp target/aarch64-skyline-switch/release/libfps_settings_plugin.nro FpsSettings/