// MACOS — capas de olas suaves que se mecen, al estilo de los fondos de macOS Big Sur.

vec3 waveColor(float k) {
    // de atrás (naranja y rosa) hacia delante (violeta y azul profundo)
    vec3 c0 = vec3(1.00, 0.68, 0.46);
    vec3 c1 = vec3(1.00, 0.47, 0.58);
    vec3 c2 = vec3(0.88, 0.37, 0.80);
    vec3 c3 = vec3(0.56, 0.33, 0.96);
    vec3 c4 = vec3(0.24, 0.36, 1.00);
    vec3 c5 = vec3(0.06, 0.20, 0.72);
    float x = clamp(k, 0.0, 1.0) * 5.0;
    if (x < 1.0) return mix(c0, c1, x);
    if (x < 2.0) return mix(c1, c2, x - 1.0);
    if (x < 3.0) return mix(c2, c3, x - 2.0);
    if (x < 4.0) return mix(c3, c4, x - 3.0);
    return mix(c4, c5, x - 4.0);
}

void mainImage(out vec4 o, vec2 fc) {
    vec2 uv = fc / iResolution.xy;
    float asp = iResolution.x / iResolution.y;
    float x = uv.x * asp;
    float t = iTime * 0.16;

    // cielo pastel
    vec3 col = mix(vec3(1.00, 0.84, 0.70), vec3(0.98, 0.70, 0.78), smoothstep(0.4, 1.0, uv.y));
    col += vec3(1.0, 0.95, 0.9) * 0.25 * exp(-length(uv - vec2(0.72, 0.92)) * 3.0);

    for (int i = 0; i < 7; i++) {
        float fi = float(i);
        float k = fi / 6.0;
        float base = 0.86 - k * 0.72;
        float h = base
            + 0.055 * sin(x * (1.1 + 0.18 * fi) + t * (0.7 + 0.12 * fi) + fi * 1.9)
            + 0.030 * sin(x * (2.6 - 0.15 * fi) - t * (0.5 + 0.09 * fi) + fi * 0.7)
            + 0.012 * sin(x * 5.3 + t * 1.3 + fi * 2.9);
        float d = uv.y - h;
        // sombra suave que la ola proyecta sobre la de detrás
        col *= 1.0 - 0.22 * exp(-max(d, 0.0) * 28.0) * step(0.0, d);
        // la ola: más clara en la cresta, más profunda abajo, con un brillo lateral
        vec3 wc = waveColor(k);
        float depth = exp(min(d, 0.0) * 5.0);
        vec3 body = wc * (0.72 + 0.38 * depth);
        body += vec3(1.0) * 0.06 * sin(x * 1.5 + fi + t * 0.8) * depth;
        body = mix(body, vec3(1.0), 0.35 * exp(-abs(d) * 220.0));
        float m = smoothstep(0.0015, -0.0015, d);
        col = mix(col, body, m);
    }

    vec2 c = uv - vec2(0.5, 0.5);
    col *= 1.0 - 0.18 * dot(c, c);
    col += grain(fc, iTime) * 0.016;
    o = vec4(col, 1.0);
}
