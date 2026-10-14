// WINDOWS — flor "Bloom" de Windows 11: pétalos azules translúcidos que respiran.

float sdVesica(vec2 p, float r, float d) {
    p = abs(p);
    float b = sqrt(r * r - d * d);
    return ((p.y - b) * d > p.x * b) ? length(p - vec2(0.0, b)) : length(p - vec2(-d, 0.0)) - r;
}

void mainImage(out vec4 o, vec2 fc) {
    vec2 R = iResolution.xy;
    vec2 p = (fc - 0.5 * R) / R.y;
    float t = iTime;

    vec2 c = vec2(0.0, -0.20);
    float dc = length(p - c);
    vec3 col = mix(vec3(0.01, 0.03, 0.10), vec3(0.03, 0.12, 0.38), exp(-dc * 1.6));
    col += vec3(0.0, 0.25, 0.8) * 0.18 * exp(-dc * 3.0);

    // dos coronas de pétalos, de atrás hacia delante
    for (int ring = 0; ring < 2; ring++) {
        float fr = float(ring);
        int n = ring == 0 ? 7 : 5;
        for (int i = 0; i < 7; i++) {
            if (i >= n) break;
            float fi = float(i);
            float spread = ring == 0 ? 2.5 : 1.7;
            float ang = -spread * 0.5 + spread * fi / float(n - 1);
            ang += 0.035 * sin(t * 0.35 + fi * 1.3 + fr * 2.0);           // respiración
            float len = (ring == 0 ? 0.56 : 0.44) * (1.0 + 0.04 * sin(t * 0.5 + fi));
            vec2 q = rot(ang) * (p - c);
            q.y -= len * 0.52;
            q.x += 0.06 * sin(q.y * 4.0 + fi) * (q.y / len);              // curvatura de cinta
            float d = sdVesica(q, len * 0.60, len * 0.43);
            float inside = smoothstep(0.003, -0.003, d);
            float along = clamp(q.y / len + 0.5, 0.0, 1.0);
            float side = clamp(q.x / (len * 0.3) * 0.5 + 0.5, 0.0, 1.0);
            vec3 deep = vec3(0.00, 0.16, 0.62);
            vec3 light = vec3(0.35, 0.78, 1.00);
            vec3 pc = mix(deep, light, along * 0.7 + side * 0.3);
            pc *= ring == 0 ? 0.75 : 1.0;
            // translúcido: se mezcla con lo que hay detrás
            col = mix(col, col * 0.35 + pc * 0.85, inside * 0.72);
            // filo luminoso del pétalo
            col += vec3(0.45, 0.85, 1.0) * exp(-abs(d) * 90.0) * (0.25 + 0.35 * along) * (ring == 0 ? 0.6 : 1.0);
        }
    }

    // luz que recorre la flor lentamente
    float sweep = sin(p.x * 2.0 - p.y * 1.0 - t * 0.25);
    col += vec3(0.25, 0.55, 1.0) * 0.06 * smoothstep(0.85, 1.0, sweep);
    col += vec3(0.5, 0.8, 1.0) * 0.18 * exp(-dc * 9.0);

    col *= 1.0 - 0.45 * dot(p, p);
    col += grain(fc, t) * 0.015;
    o = vec4(col, 1.0);
}
