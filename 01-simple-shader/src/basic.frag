precision mediump float;

// out vec4 fragColor;

uniform float u_time;
uniform vec2 canvasSize;

// attribute vec2 gl_FragCoord

// gl_Position = uProjectionMatrix * uModelViewMatrix * aVertexPosition;
void main() {
    // float r = sin(u_time * 0.0003);
    // float r = 1.0;
    // float g = sin(u_time * 0.0005);
    // float b = sin(u_time * 0.0007);

    // gl_FragColor = vec4(r, g, b, 1.0);
    // gl_text
    vec2 pos = vec2(gl_FragCoord.x / canvasSize.x, gl_FragCoord.y / canvasSize.y);

    gl_FragColor = vec4(1.,pos.x, pos.y, 1.);

}