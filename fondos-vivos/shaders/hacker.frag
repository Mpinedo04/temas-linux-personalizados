// HACKER — lluvia digital de Matrix con tres capas de profundidad.

float glyph(vec2 uv, float id) {
    if (uv.x < 0.0 || uv.x >= 1.0 || uv.y < 0.0 || uv.y >= 1.0) return 0.0;
    vec2 g = floor(uv * vec2(5.0, 7.0));
    // simétrico como los caracteres de Matrix: columnas 0 1 2 1 0
    float gx = g.x < 3.0 ? g.x : 4.0 - g.x;
    float on = step(0.52, hash21(vec2(gx + id * 7.13, g.y + id * 3.71)));
    vec2 f = fract(uv * vec2(5.0, 7.0)) - 0.5;
    return on * smoothstep(0.55, 0.30, max(abs(f.x), abs(f.y)));
}

// Devuelve el color de una capa y su brillo sin máscara (para el halo).
vec3 rainLayer(vec2 fc, float cellH, float speed, float t, float seed, out float glow) {
    vec2 cell = vec2(cellH * 0.62, cellH);
    vec2 id = floor(fc / cell);
    vec2 uv = fract(fc / cell);
    float rows = iResolution.y / cellH;
    float col = id.x + seed * 131.0;
    float fromTop = rows - id.y - 1.0;
    float exactTop = rows - fc.y / cellH;
    float bright = 0.0;
    float halo = 0.0;
    for (int k = 0; k < 2; k++) {
        float kk = float(k);
        float sp = (3.5 + 9.0 * hash11(col * 1.37 + kk * 5.1)) * speed;
        float len = 7.0 + 24.0 * hash11(col * 2.11 + kk * 9.7);
        float period = rows + len + 6.0 + 30.0 * hash11(col + kk * 3.3);
        float head = mod(t * sp + hash11(col * 7.7 + kk) * period, period);
        float d = head - fromTop;
        if (d >= 0.0 && d < len) {
            float b = pow(1.0 - d / len, 2.0);
            if (d < 1.0) b = 2.0;
            bright = max(bright, b);
        }
        float dh = head - exactTop;
        halo = max(halo, dh > 0.0 ? exp(-dh * 0.25) * step(dh, len) : exp(dh * 2.0));
    }
    // cada carácter cambia de vez en cuando
    float rate = 0.4 + 4.0 * hash21(id.yx + seed);
    float gid = floor(hash21(id + seed) * 97.0 + t * rate);
    float g = glyph((uv - vec2(0.10, 0.08)) / vec2(0.80, 0.84), gid);
    float edge = 1.0 - clamp(abs(uv.x - 0.5) * 1.8, 0.0, 1.0);
    glow = halo * edge;
    vec3 green = vec3(0.0, 1.0, 0.255);
    vec3 c = bright > 1.5 ? vec3(0.78, 1.0, 0.84) * 1.35 : green * bright;
    return c * g;
}

void mainImage(out vec4 o, vec2 fc) {
    float t = iTime;
    float H = iResolution.y;
    vec2 uv = fc / iResolution.xy;

    vec3 col = vec3(0.002, 0.016, 0.006);
    col += vec3(0.0, 0.07, 0.025) * fbm(uv * vec2(3.0, 2.0) + vec2(0.0, t * 0.02));

    float g1, g2, g3;
    vec3 far = rainLayer(fc + vec2(5.0, 0.0), H / 72.0, 0.55, t, 1.0, g1);
    vec3 mid = rainLayer(fc, H / 46.0, 0.8, t, 2.0, g2);
    vec3 near = rainLayer(fc + vec2(11.0, 0.0), H / 27.0, 1.05, t, 3.0, g3);
    col += far * 0.28 + mid * 0.62 + near;
    col += vec3(0.0, 0.85, 0.25) * (g1 * 0.03 + g2 * 0.06 + g3 * 0.09);

    // monitor CRT: líneas, viñeta y un leve parpadeo
    col *= 0.88 + 0.12 * sin(fc.y * 3.14159);
    vec2 q = uv - 0.5;
    col *= 1.0 - 0.9 * dot(q, q);
    col *= 0.97 + 0.03 * sin(t * 60.0);
    col += grain(fc, t) * 0.025;
    o = vec4(col, 1.0);
}
