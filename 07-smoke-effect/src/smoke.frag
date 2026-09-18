// ---- smoke.frag ----
precision highp float;

varying vec2 vUV;

uniform float u_time;
uniform vec2  canvasSize;
const vec3  uColor = vec3(0.8, 0.8, 0.85);        // base smoke tint (e.g. vec3(0.8, 0.8, 0.85))
const float uDensity = 12.9;      // overall opacity  (0.0 – 1.0)
const float uSpeed = 0.5;        // scroll speed
const float uScale = 10.4;        // noise scale (higher = tighter features)
const float uOctaves = 3.0;      // FBM layers (3–7 looks good)
const float uPersistence = 0.4;  // 0.5 – 0.7

// ---------- 2D hash / noise ----------
float hash(vec2 p) {
    p = fract(p * vec2(123.34, 456.21));
    p += dot(p, p + 45.32);
    return fract(p.x * p.y);
}

float noise(vec2 p) {
    vec2 i = floor(p);
    vec2 f = fract(p);
    vec2 u = f * f * (3.0 - 2.0 * f); // smoothstep

    float a = hash(i);
    float b = hash(i + vec2(1.0, 0.0));
    float c = hash(i + vec2(0.0, 1.0));
    float d = hash(i + vec2(1.0, 1.0));

    return mix(mix(a, b, u.x),
               mix(c, d, u.x), u.y);
}

// ---------- Fractal Brownian Motion ----------
float fbm(vec2 p) {
    float value    = 0.0;
    float amplitude = 0.5;
    float frequency = 1.0;

    for (int i = 0; i < 7; i++) {
        if (float(i) >= uOctaves) break;
        value += amplitude * noise(p * frequency);
        frequency  *= 2.0;
        amplitude  *= uPersistence;
    }
    return value;
}

// ---------- Main ----------
void main() {
    // Aspect-correct UVs
    // vec2 uv = vUV;
    // vec2 pos = vec2(gl_FragCoord.x / canvasSize.x, gl_FragCoord.y / canvasSize.y);
    vec2 uv = vec2(gl_FragCoord.x / canvasSize.x, gl_FragCoord.y / canvasSize.y);
    // uv.x *= canvasSize.x / canvasSize.y;

    // Scroll upward (smoke rises)
    vec2 scroll = vec2(0.0, u_time * uSpeed);

    // Layer two FBM samples for a more organic swirl
    float n1 = fbm(uv * uScale + scroll);
    float n2 = fbm(uv * uScale * 1.7 - scroll * 0.6 + n1 * 2.0);

    // Combine and shape
    float smoke = n1 * 0.6 + n2 * 0.4;

    // Soften & push toward the edges (dissipate)
    float edge  = smoothstep(0.0, 0.15, uv.x)
                * smoothstep(1.0, 0.85, uv.x)
                * smoothstep(0.0, 0.20, uv.y)
                * smoothstep(1.0, 0.70, uv.y);

    // float alpha = smoothstep(0.35, 0.75, smoke) * edge * uDensity;
    float alpha = smoothstep(0.25, 0.85, smoke) * edge * uDensity;
    // alpha *= step(0.2, alpha); 

    // Slight brightness variation for depth
    float brightness = 0.1 + 0.9 * n2;
    vec3 col = uColor * brightness;

    gl_FragColor = vec4(col, alpha);
    // gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0);
    // Discard fully transparent fragments for cheaper blending
    // if (alpha < 0.01) discard;
}
