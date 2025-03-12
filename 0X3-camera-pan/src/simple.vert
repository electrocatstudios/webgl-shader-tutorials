precision mediump float;

attribute vec3 a_position;
attribute vec3 a_normal;
attribute vec2 a_texcoord;
attribute vec3 a_color;

uniform float u_time;
uniform vec2 u_screensize;
uniform mat4 u_projection;
uniform mat4 u_view;
uniform mat4 u_model;

varying vec3 v_normal;
varying vec2 v_texcoord;
varying vec3 v_color;
varying float v_time;

void main() {
    // Pass through varyings
    v_normal = (u_model * vec4(a_normal, 0.0)).xyz;
    v_texcoord = a_texcoord;
    v_color = a_color;
    v_time = u_time;

    // Adjust the position based on the screen size
    vec4 worldPosition = u_model * vec4(a_position, 1.0);
    vec4 viewPosition = u_view * worldPosition;
    vec4 projectedPosition = u_projection * viewPosition;
    
    // Apply screen size correction
    vec2 uv = projectedPosition.xy; 
    // uv.x /= u_screensize.x / u_screensize.y;
    gl_Position = vec4(uv, projectedPosition.z, projectedPosition.w);
}