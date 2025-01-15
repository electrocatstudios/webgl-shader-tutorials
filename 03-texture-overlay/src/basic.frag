precision mediump float;

uniform float u_time;
uniform vec2 canvasSize;
uniform sampler2D texNoise;

void main(void)
{
  vec2 uv = vec2(gl_FragCoord.x / canvasSize.x, gl_FragCoord.y / canvasSize.y);
  uv += vec2(0.2 * sin(u_time), 0.2 * cos(u_time));
  
  float sin_res = (sin(u_time) / 2.) + 0.5;
  float cos_res = (cos(u_time) / 2.) + 0.5;
  
  vec3 col = vec3(sin_res*uv.x,clamp(sin_res*uv.y, 0.2, 1.0),clamp(cos_res*uv.x,0.4,1.0));
  vec2 q = uv - vec2(0.5,0.5);
  float r = 0.15 + ( 0.1 * ( sin(u_time) * cos(  sin(u_time * 0.8) *  atan( q.y, q.x) * 10.0 ) ) );
  
  col *= smoothstep(r, r+0.05, length(q));
  
  vec2 texUV = vec2(gl_FragCoord.x / canvasSize.x, gl_FragCoord.y / canvasSize.y);
  texUV.x -= (sin(u_time) * 0.4);
  texUV.y -= (cos(u_time) * 0.4); 
  vec4 out_col = texture2D(texNoise, texUV);
  
  vec3 out_col_new = vec3(
          clamp(out_col.x, 0., 1.0) + 0.0,
          clamp(out_col.y, 0., 1.0) + 0.0,
          clamp(out_col.z, 0., 1.0) + 0.0
          );
  vec4 out_col1 = vec4(out_col_new * col, 1.0);
  
  gl_FragColor = ( out_col1);
}