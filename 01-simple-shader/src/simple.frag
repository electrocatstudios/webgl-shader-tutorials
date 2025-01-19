precision mediump float;

uniform float u_time;
uniform vec2 canvasSize;

void main() {
    
    vec2 pos = vec2(gl_FragCoord.x / canvasSize.x, gl_FragCoord.y / canvasSize.y);
    float t = (sin(u_time) * 0.5) + 0.5;
    gl_FragColor = vec4(t,pos.x * ((cos(u_time) * 0.5) + 0.5) , pos.y, 1.);

}