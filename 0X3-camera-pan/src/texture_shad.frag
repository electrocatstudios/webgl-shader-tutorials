precision mediump float;

uniform sampler2D u_texture;

varying vec3 v_normal;
varying vec2 v_texcoord;

varying vec3 v_color;
varying float v_time;

uniform vec4 u_baseColorFactor;

void main() {
    // Simple diffuse lighting
    vec3 normal = normalize(v_normal);
    vec3 lightDir = normalize(vec3(1.0, 1.0, 0.0));
    float diffuse = max(dot(normal, lightDir), 0.1);
    
    // Use material color or fallback to light gray
    vec4 baseColor = u_baseColorFactor;

    float blue = (sin(v_time * 1.2) / 2.0) + 0.5;
    blue = clamp(blue, 0.0, 1.0);
    vec3 col = vec3(v_texcoord, sin(v_time));
    //col += texture2D(u_texture, v_texcoord.xy).rgb;
    col = texture2D(u_texture, v_texcoord.xy).rgb;
    gl_FragColor = vec4( col, 1.0);
    // gl_FragColor = vec4(v_color, 1.0);
}