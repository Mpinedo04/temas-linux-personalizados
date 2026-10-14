// MINIMALISTA — aurora boreal sobre montañas nevadas, con los colores de Nord. Lenta y tranquila.

float ridge(float x, float seed) {
    float v = 0.0;
    float a = 0.5;
    float f = 1.0;
    for (int i = 0; i < 5; i++) {
        float n = noise(vec2(x * f + seed, seed * 1.7));
        v += a * (1.0 - abs(n * 2.0 - 1.0));
        f *= 2.1;
        a *= 0.48;
    }
    return v;
}

void mainImage(out vec4 o, vec2 fc) {
    vec2 R = iResolution.xy;
    vec2 uv = fc / R;
    float asp = R.x / R.y;
    vec2 p = vec2(uv.x * asp, uv.y);
    float t = iTime;

    vec3 night = vec3(0.141, 0.161, 0.200);  // #242933
    vec3 polar = vec3(0.180, 0.204, 0.251);  // #2E3440
    vec3 col = mix(polar * 0.95, night * 0.7, smoothstep(0.15, 1.0, uv.y));

    // estrellas que titilan
    vec2 sg = floor(p * 230.0);
    float st = hash21(sg);
    vec2 sf = fract(p * 230.0) - 0.5;
    float star = step(0.993, st) * smoothstep(0.4, 0.0, length(sf));
    col += vec3(0.925, 0.937, 0.957) * star * (0.3 + 0.7 * (0.5 + 0.5 * sin(t * (0.5 + st * 2.0) + st * 40.0))) * smoothstep(0.35, 0.75, uv.y);

    // aurora: cortinas con rayos verticales que ondulan
    vec3 aurora = vec3(0.0);
    for (int i = 0; i < 3; i++) {
        float fi = float(i);
        float base = 0.47 + 0.09 * fi
                   + 0.07 * sin(p.x * 2.2 + t * 0.07 + fi * 2.1)
                   + 0.035 * sin(p.x * 5.1 - t * 0.11 + fi * 1.3)
                   + 0.015 * sin(p.x * 11.0 + t * 0.2 + fi)
                   + 0.08 * (fbm(vec2(p.x * 1.4 + t * 0.03, fi * 3.0)) - 0.5);
        float dy = uv.y - base;
        float curtain = smoothstep(-0.09, 0.025, dy) * exp(-max(dy, 0.0) * (5.0 + fi * 3.0));
        curtain *= smoothstep(0.5, 0.05, dy);
        float rays = pow(fbm(vec2(p.x * 16.0 + fi * 7.0 - t * 0.06, t * 0.1 + fi)), 1.5) * 1.6;
        float pulse = 0.7 + 0.3 * sin(t * 0.25 + fi * 1.7 + p.x * 0.9);
        vec3 c1 = mix(vec3(0.561, 0.737, 0.733), vec3(0.639, 0.745, 0.549), fi * 0.5); // #8FBCBB → #A3BE8C
        vec3 c2 = vec3(0.706, 0.557, 0.678);                                          // #B48EAD
        vec3 cc = mix(c1 * 1.25, c2, smoothstep(0.06, 0.3, dy) * 0.75);
        aurora += cc * curtain * rays * pulse * (0.75 - 0.15 * fi);
    }
    col += aurora;
    // resplandor de la aurora en el cielo bajo
    col += vec3(0.561, 0.737, 0.733) * 0.10 * smoothstep(0.65, 0.3, uv.y) * smoothstep(0.15, 0.4, uv.y);

    // montañas: cordillera lejana con nieve y otra cercana más oscura
    float m1 = 0.20 + 0.30 * ridge(p.x * 1.1, 3.0);
    float m2 = 0.07 + 0.16 * ridge(p.x * 1.8 + 7.0, 9.0);
    vec3 far = vec3(0.263, 0.298, 0.369);   // #434C5E
    far = mix(far, vec3(0.847, 0.871, 0.914) * 0.85, smoothstep(m1 - 0.05, m1 - 0.005, uv.y) * smoothstep(0.36, 0.44, m1)); // nieve #D8DEE9
    far += aurora * 0.12;
    vec3 near = vec3(0.180, 0.204, 0.251) * 0.85;
    col = mix(col, far, smoothstep(m1 + 0.0015, m1 - 0.0015, uv.y));
    col = mix(col, near, smoothstep(m2 + 0.0015, m2 - 0.0015, uv.y));
    // niebla en el valle
    col += vec3(0.36, 0.40, 0.48) * 0.12 * exp(-abs(uv.y - m2 - 0.015) * 30.0) * (0.6 + 0.4 * noise(vec2(p.x * 3.0 + t * 0.05, 0.0)));

    col += grain(fc, t) * 0.015;
    o = vec4(col, 1.0);
}
