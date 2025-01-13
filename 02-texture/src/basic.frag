precision mediump float;

uniform float u_time;
uniform vec2 canvasSize;
uniform sampler2D texNoise;

void main() {
    
    vec2 pos = vec2(gl_FragCoord.x / canvasSize.x, gl_FragCoord.y / canvasSize.y);
    float t = (sin(u_time) * 0.5) + 0.5;
    vec4 texVal = texture(texNoise, texUV);
    gl_FragColor = vec4(t,pos.x * ((cos(u_time) * 0.5) + 0.5) , pos.y, 1.) * texVal;

}