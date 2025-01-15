// Taken from here: https://www.shadertoy.com/view/4djBRm
precision mediump float;

const float TUNNEL_SIZE  = 0.25;	// smaller values for smaller/thinner tunnel
const float TUNNEL_SPEED = 0.5;		// speed of tunnel effect, negative values ok

const float PI = 3.141592;

uniform float u_time;
uniform vec2 canvasSize;
uniform sampler2D texNoise;

vec2 tunnel(vec2 uv, float size, float time)
{
    vec2 p  = -1.0 + (2.0 * uv);
    float a = atan(p.y, p.x);
    float r = sqrt(dot(p, p));
    return vec2(a / PI, time + (size / r));
}

void main()
{
	vec2 uv = vec2(gl_FragCoord.x / canvasSize.x, gl_FragCoord.y / canvasSize.y);
  uv = tunnel(uv, TUNNEL_SIZE, u_time * TUNNEL_SPEED);
	gl_FragColor = texture2D(texNoise, uv);
}
