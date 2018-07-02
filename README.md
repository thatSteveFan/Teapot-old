# Teapot-old
This was a simple test of my ability to learn Rust. It was my first forray into Rust outside of TRPL, 
and my first time dipping my toe into 3D graphics. I have abandonded it as the author of the OpenGL library I am using has abandoned their library. 
I will be rewriting it in Vulkan at https://github.com/thatSteveFan/Teapot-Vulkan. 

Controls:
WASD move the camera in the y and z directions respectively. Scrolling rotates the camera on the Z-axis, and touchscreen swipes rotate in the other directions (unfortunately mouse events are more complicated to track between loops).    

All the OpenGL calls are done with the [GLium library](https://github.com/glium/glium)

Large portions of code are taken from the [tutorial in the glium library](https://github.com/glium/glium/tree/master/book). It is an excelent tutorial that will 
teach the basics well.

Additional thanks to my friend Emma for help with understanding the Linear Algebra at the heart of all of this, and to various other friends for 
listening to be ramble about "my teapot" for hours on end.