// #version 300 es

// in vec3 vertexPosition;

// void main() {
//   gl_Position = vec4(vertexPosition, 1.0);
// }
precision mediump float;

attribute vec2 a_position;

void main() {
    gl_Position = vec4(a_position, 0.0, 1.0);
}