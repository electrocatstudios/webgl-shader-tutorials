precision highp float;

uniform float u_time;
uniform vec2 canvasSize;
uniform sampler2D texture;
uniform float mouse_x;
uniform float mouse_y;

#define PI 3.1415
#define TWO_PI 6.2824

// Details for the fractal max
#define DEPTH_COUNT 10

const int MAX_FRACTAL_DEPTH = DEPTH_COUNT;
const float max_num = float(DEPTH_COUNT);

vec2 N(float angle) {
  return vec2(sin(angle), cos(angle));
}

void main(void)
{
  vec2 uv = (gl_FragCoord.xy-(.5*canvasSize.xy)) / canvasSize.y;
  uv *= 2.;
  
  float angle = (5./6.) * PI;
  uv.y -= 0.25; // Fiddle factor to recenter the image
  
  vec3 col = vec3(0.);

  uv.x = abs(uv.x); // Reflect around center line
  
  vec2 n = N(angle);
  
  float d = dot(uv-vec2(0.5,0.0), n); // Offset the reflection
  uv -= n*max(0., d)*2.; // Do the reflection

  // DEBUG - uncomment to show the lines of where reflection is ocurring
  // col += smoothstep(.01, .0, abs(d));

  float scale = 2.;
  
  // Calculate depth of max number of iterations
  // We cycle between low depth and high depth
  // - more depth means more recursions of the fractal pattern
  float speed = 2.0 + (8. * mouse_y);

  float num_calc = max_num * mouse_y; //mod( (u_time / speed) * (max_num / 2.), max_num);
  num_calc -= max_num / 2.0;
  num_calc = abs(num_calc) * 2.0;
  num_calc -= 5.; 
  num_calc = clamp(num_calc, 0., 10.);
  int num = int(floor(num_calc));

  // Go to original angle to draw the top bar
  n = N((2./3.) * PI);
  uv.x += 0.5;

  // Have to use a const for a for loop
  for(int i=0;i<MAX_FRACTAL_DEPTH;i++){
    // Apply the fractal, dividing up into 3
    uv *= 3.;
    uv.x -= 1.5;
    scale *= 3.;  
  
    uv.x = abs(uv.x);
    uv.x -= 0.5;
    uv -= n*min(0., dot(uv, n))*2.;

    // This is how we dynamically break the loop
    if(i >= num){
      break;
    }
  }
      
  d = length(uv - vec2(clamp(uv.x, -1.,1.), 0.));

  col += smoothstep(1./canvasSize.y, .0, d/scale);
  col.rg += uv * 0.1;
  
  // Red hues
  // col.r += uv.x * 0.1;
  // col.g += (1.0-uv.y) * 0.1;

  col.b = 0.6;
  uv /= scale;
  col += texture2D(texture, uv*2.+(u_time*.1)).rgb;
  col *= (mouse_x * 0.8) + 0.1;
  gl_FragColor = vec4(col, 1.);
}