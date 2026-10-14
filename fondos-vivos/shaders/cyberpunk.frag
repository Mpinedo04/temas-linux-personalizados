// CYBERPUNK — Night City: sol synthwave, rascacielos de neón, coches voladores,
// lluvia, suelo de rejilla en movimiento y "glitches" ocasionales.

const vec3 MAGENTA = vec3(1.0, 0.17, 0.84);
const vec3 CYAN = vec3(0.0, 0.94, 1.0);
const vec3 YELLOW = vec3(0.99, 0.93, 0.04);

vec3 sky(float y) {
    vec3 top = vec3(0.015, 0.008, 0.06);
    vec3 mid = vec3(0.14, 0.02, 0.22);
    vec3 hor = vec3(0.75, 0.08, 0.50);
    vec3 c = mix(hor, mid, smoothstep(0.0, 0.28, y));
    return mix(c, top, smoothstep(0.25, 1.0, y));
}

// Una capa de edificios. Devuelve 1 si el punto está dentro de un edificio.
float skyline(vec2 p, float horizon, float w, float hMin, float hMax, float seed,
              out float id, out vec2 local, out float top) {
    float bx = floor(p.x / w);
    id = bx + seed * 97.0;
    float r = hash11(id * 1.731);
    top = horizon + hMin + (hMax - hMin) * r * r;
    // algunos edificios con escalón o antena
    float step2 = hash11(id * 4.1) > 0.6 ? 0.25 + 0.5 * hash11(id * 2.3) : 0.0;
    float fx = fract(p.x / w);
    float t2 = top;
    if (step2 > 0.0 && abs(fx - 0.5) > step2 * 0.5) t2 = top - (top - horizon) * 0.18;
    local = vec2(fx, (p.y - horizon) / max(top - horizon, 0.001));
    float gap = smoothstep(0.0, 0.04, fx) * smoothstep(1.0, 0.96, fx);
    return step(p.y, t2) * step(horizon - 0.01, p.y) * step(0.5, gap + 0.5);
}

vec3 windows(vec2 p, float id, float w, float t, float dim) {
    vec2 g = vec2(w / 7.0, 0.011);
    vec2 cell = floor(p / g);
    vec2 f = fract(p / g);
    float lit = step(0.62, hash21(cell + id));
    float flick = step(0.04, hash21(cell * 1.3 + floor(t * (0.3 + hash11(id)))));
    float shape = step(0.18, f.x) * step(f.x, 0.82) * step(0.25, f.y) * step(f.y, 0.75);
    float pick = hash21(cell.yx + id * 3.0);
    vec3 c = pick < 0.45 ? vec3(1.0, 0.75, 0.35) : (pick < 0.75 ? CYAN : MAGENTA);
    return c * lit * flick * shape * dim;
}

void mainImage(out vec4 o, vec2 fc) {
    vec2 R = iResolution.xy;
    vec2 uv = fc / R;
    float asp = R.x / R.y;
    float t = iTime;

    // glitch: unas pocas veces por minuto, durante un instante
    float gT = floor(t * 1.6);
    float gOn = step(0.95, hash11(gT * 3.17)) * step(fract(t * 1.6), 0.3);
    float band = floor(uv.y * 30.0);
    float shift = (hash21(vec2(band, gT)) - 0.5) * 0.07 * gOn * step(0.55, hash21(vec2(gT, band * 1.7)));
    vec2 p = vec2((uv.x - 0.5) * asp + shift, uv.y);

    float horizon = 0.34;
    vec3 col;

    if (p.y >= horizon) {
        float sy = (p.y - horizon) / (1.0 - horizon);
        col = sky(sy);
        // nubes de neblina iluminadas por la ciudad
        col += vec3(0.35, 0.05, 0.30) * fbm(vec2(p.x * 1.5 + t * 0.01, p.y * 3.0)) * (1.0 - sy) * 0.6;
        // estrellas
        vec2 sg = floor(p * 160.0);
        float st = hash21(sg);
        col += vec3(0.85, 0.75, 1.0) * step(0.996, st) * (0.4 + 0.6 * sin(t * 2.5 + st * 60.0)) * smoothstep(0.35, 0.9, sy);

        // sol synthwave con franjas
        vec2 sc = vec2(0.0, horizon + 0.19);
        float sr = 0.21;
        float d = length(p - sc);
        float k = clamp((p.y - (sc.y - sr)) / (2.0 * sr), 0.0, 1.0);
        vec3 sun = mix(vec3(1.0, 0.12, 0.55), vec3(1.0, 0.86, 0.25), k);
        float lines = step(0.42 + 0.35 * (1.0 - k), fract((p.y - sc.y) * 34.0 + t * 0.35));
        float cut = mix(1.0, 1.0 - lines, smoothstep(0.62, 0.2, k));
        col = mix(col, sun, cut * smoothstep(sr, sr - 0.003, d));
        col += vec3(1.0, 0.2, 0.55) * 0.42 * exp(-max(d - sr, 0.0) * 7.0) * (0.9 + 0.1 * sin(t * 0.8));

        // coches voladores con estela
        for (int i = 0; i < 7; i++) {
            float fi = float(i);
            float h = horizon + 0.22 + 0.55 * hash11(fi * 3.7);
            float dir = hash11(fi * 9.1) > 0.5 ? 1.0 : -1.0;
            float sp = (0.06 + 0.14 * hash11(fi * 1.3)) * dir;
            float x = (mod(t * sp + hash11(fi * 5.5) * 6.0, 6.0) - 3.0) * asp * 0.5;
            vec2 dd = p - vec2(x, h + 0.006 * sin(t * 1.3 + fi));
            float lamp = exp(-dot(dd * vec2(1.0, 2.5), dd * vec2(1.0, 2.5)) * 14000.0);
            float tail = exp(-abs(dd.y) * 900.0) * (dd.x * dir < 0.0 ? exp(dd.x * dir * 9.0) : 0.0);
            vec3 cc = hash11(fi * 2.2) > 0.5 ? CYAN : vec3(1.0, 0.25, 0.3);
            col += cc * (lamp * 1.5 + tail * 0.5);
        }

        // tres capas de rascacielos
        float id; vec2 lc; float top;
        if (skyline(p, horizon, 0.045, 0.06, 0.22, 1.0, id, lc, top) > 0.5) {
            col = mix(col, vec3(0.20, 0.05, 0.28), 0.85);
            col += windows(p, id, 0.045, t, 0.35);
        }
        if (skyline(p + vec2(0.37, 0.0), horizon, 0.07, 0.03, 0.34, 2.0, id, lc, top) > 0.5) {
            col = vec3(0.07, 0.02, 0.11) + windows(p, id, 0.07, t, 0.6);
            // borde de neón en lo alto
            col += MAGENTA * exp(-(top - p.y) * 250.0) * step(0.7, hash11(id * 6.1)) * 0.8;
        }
        if (skyline(p + vec2(1.13, 0.0), horizon, 0.11, 0.0, 0.48, 3.0, id, lc, top) > 0.5) {
            col = vec3(0.02, 0.006, 0.035) + windows(p, id, 0.11, t, 0.9);
            // letreros verticales de neón que parpadean
            float neon = step(0.55, hash11(id * 8.3));
            float sx = abs(lc.x - (0.2 + 0.6 * hash11(id * 2.9)));
            float sv = step(sx, 0.035) * step(0.35, lc.y) * step(lc.y, 0.8);
            float on = step(0.12, hash11(floor(t * 3.0) + id));
            vec3 sc2 = hash11(id * 4.4) > 0.5 ? CYAN : MAGENTA;
            col += sc2 * neon * sv * on * 1.2;
            col += sc2 * neon * exp(-sx * 40.0) * step(0.3, lc.y) * step(lc.y, 0.85) * on * 0.25;
            // luz roja en la antena
            float ant = step(abs(lc.x - 0.5), 0.02) * step(top, p.y + 0.02);
            col += vec3(1.0, 0.1, 0.1) * ant * step(0.5, fract(t * 0.8 + hash11(id)));
        }
    } else {
        // suelo: rejilla de neón en perspectiva, mojada por la lluvia
        float dy = horizon - p.y;
        float z = 0.18 / (dy + 0.004);
        vec2 g = vec2(p.x * z * 1.6, z + t * 1.3);
        vec2 gw = fwidth(g);
        vec2 gl = abs(fract(g) - 0.5) / gw;
        float line = 1.0 - clamp(min(gl.x, gl.y) * 0.7, 0.0, 1.0);
        float fade = exp(-z * 0.08);
        col = mix(vec3(0.03, 0.005, 0.06), vec3(0.10, 0.01, 0.12), fade);
        col += mix(CYAN, MAGENTA, smoothstep(0.0, 0.3, dy)) * line * fade * 1.3;
        // reflejo del sol y de la ciudad en el suelo mojado
        float refl = exp(-abs(p.x) * 6.0) * exp(-dy * 5.0);
        col += vec3(1.0, 0.2, 0.55) * refl * 0.45 * (0.8 + 0.2 * sin(dy * 300.0 - t * 4.0));
        col += vec3(0.6, 0.1, 0.5) * exp(-dy * 18.0) * 0.5;
    }

    // lluvia en dos capas
    for (int l = 0; l < 2; l++) {
        float fl = float(l);
        vec2 rp = vec2(p.x, uv.y) * vec2(70.0 + 50.0 * fl, 3.0 + fl);
        rp.y += t * (5.0 + 3.0 * fl);
        rp.x += rp.y * 0.16;
        vec2 rid = floor(rp);
        vec2 rf = fract(rp);
        float drop = step(0.93, hash21(rid + fl * 13.0));
        drop *= smoothstep(0.08, 0.0, abs(rf.x - 0.5)) * smoothstep(0.0, 0.7, rf.y) * smoothstep(1.0, 0.7, rf.y);
        col += vec3(0.55, 0.6, 1.0) * drop * (0.16 + 0.08 * fl);
    }

    // glitch: separación de color y líneas de interferencia
    col += (MAGENTA * step(0.5, hash21(vec2(band, gT * 1.3))) + CYAN * step(0.85, hash21(vec2(gT, band)))) * gOn * 0.25;

    // pantalla: líneas, viñeta, grano
    col *= 0.9 + 0.1 * sin(fc.y * 2.2);
    vec2 q = uv - 0.5;
    col *= 1.0 - 0.75 * dot(q, q);
    col += grain(fc, t) * 0.03;
    o = vec4(col, 1.0);
}
