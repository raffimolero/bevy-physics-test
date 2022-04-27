# bevy-physics-test

Created to test 3D development using Bevy.
Currently, it simulates several blue spheres attracted to each other through gravity and bouncing off of each other as in 3D billiards.

There is one red sphere in the center of the scene; this sphere has no collision, and serves only as a marker.
There is one green sphere that follows your mouse if it is on-screen.
You may use this object to hit other spheres. The strength of the bounce depends on the speed of your mouse.
If your mouse is off-screen, the green sphere will simply act according to physics.

Controls:
- Camera
  - Forward/Backward: `W`/`S`
  - Left/Right: `A`/`D`
  - Up/Down: `Space`/`Shift`
  - Decrease/Increase Fly Speed: `<` and `>`
- Physics
  - Decrease/Increase Gravity: `-`/`+`
