precision highp float;

uniform float u_time;
uniform vec2 canvasSize;
uniform sampler2D texNoise;

#define PI 3.1412
#define TWO_PI 6.2824

// Details for the fractal max
#define DEPTH_COUNT 20

const int MAX_FRACTAL_DEPTH = DEPTH_COUNT;
const float max_num = float(DEPTH_COUNT);

void main(void)
{
  vec2 uv = ( gl_FragCoord.xy * 2.0 - canvasSize.xy ) / canvasSize.y;
  vec3 col = vec3(0.);

  float angle = (2./3.)*PI;
  vec2 n = vec2(sin(angle), cos(angle));
  
  float scale = 10.;
  
  float num_calc = mod(u_time * (max_num / 2.), max_num);
 
  num_calc -= max_num / 2.0;
  num_calc = abs(num_calc) * 2.0;
  num_calc -= 5.; 
  num_calc = clamp(num_calc, 1., 10.);
  int num = int(floor(num_calc));

  uv.x += 0.5;
  for(int i=0;i<MAX_FRACTAL_DEPTH;i++){
    uv *= 3.;
    uv.x -= 1.5;
    scale *= 3.;  
  
    uv.x = abs(uv.x);
    uv.x -= 0.5;
    uv -= n*min(0., dot(uv, n))*2.;

    if(i >= num){
      break;
    }
  }
      
  float d = length(uv - vec2(clamp(uv.x, -1.,1.), 0.));
  col += smoothstep(1./canvasSize.y,.0, d/scale);
  col.rg += uv *0.1;
  col.b = 0.6;
    
  gl_FragColor = vec4(col, 1.);
}