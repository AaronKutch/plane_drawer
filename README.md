# Silly-Cam
draws 2D shapes relative to draw planes, which can be rotated only about the z axis and exist in 3D space (so 2.5D). Given camera position and zoom, writes shapes to a 2D buffer (note: does not actually draw to screen)

Included is an example file that contains a minifb update loop which draws some example shapes to screen and handles camera controls including zoom.
also includes a half-baked geometry library. Note that this is not supposed to be a production-ready general purpose library, it was originally created for a very specific purpose.