# RPI4 Led Controller
A simple MQTT controlled LED strip controller

## Animations
The led controller contains multiple animations that can be contolled through an mqtt broker.

* Rainbow; A simple circular rainbow animation, built from a increasing angle on a hue wheel.
* Chase; A chase animation, with a build-up at the end of the strip, then the animation is reversed and the leds are progresively turned off.
* Static rainbow; Similar to the rainbow animation but the color is the same at a given time on the whole strip.

## Hardware
A raspberrypi 4 - 4Gb is used, along a ws2812b rgb led strip. Some part of the strip in beneath my `Ender 5` 3D printer, the other part on the inside of the front-top bar, to light up the printing plate. This is why the LED is separated in the code into *WHEEL* and *STRIP*. This allows to control the two parts separately (e.g. keep a white light on the plate but a rainbow on the rest of the printer).

The data-lane of the strip is connected to the pin 18 on the GPIO of the raspberrypi.

## Software
The software is simply run as a service on the raspberrypi
```toml
[Unit]
Description="LED controller for 3D printer's strip"

[Service]
ExecStart=<path to the build>
Restart=on-success
Type=simple

[Install]
WantedBy=multi-user.target
```