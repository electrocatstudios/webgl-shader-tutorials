// Taken from here: https://www.shadertoy.com/view/4djBRm
precision highp float;

const float TUNNEL_SIZE  = 0.25;	// smaller values for smaller/thinner tunnel
const float TUNNEL_SPEED = 0.5;		// speed of tunnel effect, negative values ok

const float PI = 3.141592;

uniform float u_time;
uniform vec2 canvasSize;
uniform sampler2D texNoise;
uniform float red;
uniform float green;
uniform float blue;

vec2 tunnel(vec2 uv, float size, float time)
{
    vec2 p  = -1.0 + (2.0 * uv);
    float a = atan(p.y, p.x);
    float r = sqrt(dot(p, p));
    return vec2(a / PI, time + (size / r));
}

vec3 palette(float t, vec3 a, vec3 b, vec3 c, vec3 d) {
  return a + b*cos(6.28318*(c*t+d));
}

float circle_dist(vec2 start, vec2 end) {
  vec2 calc = vec2(start.x - end.x, start.y - end.y);
  return length(start - end);
}

float dis=.5;
float width=.1;
float blur=.3;

void main()
{
	vec2 uv = ( gl_FragCoord.xy * 2.0 - canvasSize.xy ) / canvasSize.y;
  
  //vec2 o = uv;
  
  float angle = atan( uv.y, uv.x );
  
  float l = length(uv);
  
  float offset = ( log(l) + ( angle / ( 2.*PI ) ) * dis );
  float circles = mod( offset + u_time, dis );
  
  vec3 lineColor = vec3(red, green, blue);

  vec3 col = ( smoothstep( circles-blur, circles, width ) - smoothstep( circles, circles+blur, width ) ) * lineColor;

  gl_FragColor = vec4(col, 1.0);
}
