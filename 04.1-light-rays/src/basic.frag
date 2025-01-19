// Taken from here: https://www.shadertoy.com/view/4djBRm
precision mediump float;

const float TUNNEL_SIZE  = 0.25;	// smaller values for smaller/thinner tunnel
const float TUNNEL_SPEED = 0.025;		// speed of tunnel effect, negative values ok

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
	vec2 uv = vec2(canvasSize.y / canvasSize.x, 1.);
  uv *= vec2(gl_FragCoord.x, gl_FragCoord.y);
  uv = tunnel(uv, TUNNEL_SIZE, u_time * TUNNEL_SPEED);
	vec4 col = texture2D(texNoise, uv);
  gl_FragColor = col * vec4(0.4,0.4,0.9,1.0);
}
