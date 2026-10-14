// Transición entre el fondo animado anterior (uOld) y el nuevo (uNew).
// uKind elige el efecto según el estilo de destino:
// 0 hacker · 1 cyberpunk · 2 macos · 3 windows · 4 minimalista · otro = fundido.

uniform sampler2D uOld;
uniform sampler2D uNew;
uniform float uProgress;
uniform int uKind;
in vec2 vUv;
out vec4 fragColor;

vec3 A(vec2 uv) { return texture(uOld, uv).rgb; }
vec3 B(vec2 uv) { return texture(uNew, uv).rgb; }

void main() {
    vec2 uv = vUv;
    float p = clamp(uProgress, 0.0, 1.0);
    vec3 c;

    if (uKind == 0) {
        // lluvia digital: cada columna cae a su ritmo y descubre el fondo nuevo
        float cols = 90.0;
        float id = floor(uv.x * cols);
        float delay = hash11(id * 1.7) * 0.45;
        float pp = clamp((p - delay) / 0.55, 0.0, 1.0);
        float front = 1.0 - pp * 1.15;
        float m = step(front, uv.y);
        c = mix(A(uv), B(uv), m);
        float edge = exp(-abs(uv.y - front) * 140.0) * step(0.001, pp) * step(pp, 0.999);
        c += vec3(0.6, 1.0, 0.7) * edge * 1.5 + vec3(0.0, 1.0, 0.3) * exp(-max(uv.y - front, 0.0) * 25.0) * m * (1.0 - pp) * 0.5;
    } else if (uKind == 1) {
        // glitch de neón: bloques que saltan, franjas desplazadas y separación RGB
        float s = sin(p * 3.14159);
        float tick = floor(p * 18.0);
        float bandId = floor(uv.y * 36.0);
        float off = (hash21(vec2(bandId, tick)) - 0.5) * 0.09 * s * step(0.45, hash21(vec2(tick, bandId * 1.3)));
        vec2 blk = floor((uv + vec2(off, 0.0)) * vec2(28.0, 16.0));
        float m = step(hash21(blk + tick * 0.17), p * 1.15 - 0.05);
        float k = 0.012 * s;
        vec2 u = uv + vec2(off, 0.0);
        c.r = mix(A(u + vec2(k, 0.0)).r, B(u + vec2(k, 0.0)).r, m);
        c.g = mix(A(u).g, B(u).g, m);
        c.b = mix(A(u - vec2(k, 0.0)).b, B(u - vec2(k, 0.0)).b, m);
        float flash = step(0.97, hash21(vec2(bandId, tick * 2.1)));
        c += mix(vec3(1.0, 0.17, 0.84), vec3(0.0, 0.94, 1.0), hash21(vec2(tick, bandId))) * flash * s * 0.8;
        c *= 0.92 + 0.08 * sin(uv.y * 900.0);
    } else if (uKind == 2) {
        // macOS: fundido con un zoom muy suave
        float e = smoothstep(0.0, 1.0, p);
        vec2 ua = (uv - 0.5) * (1.0 - 0.05 * e) + 0.5;
        vec2 ub = (uv - 0.5) * (1.0 + 0.05 * (1.0 - e)) + 0.5;
        c = mix(A(ua), B(ub), e);
        c = mix(c, vec3(1.0), 0.12 * sin(e * 3.14159));
    } else if (uKind == 3) {
        // Windows: barrido diagonal con un filo de luz azul
        float d = uv.x * 0.75 + (1.0 - uv.y) * 0.25;
        float f = p * 1.35 - 0.2;
        float m = smoothstep(f + 0.05, f - 0.05, d);
        c = mix(A(uv), B(uv), m);
        c += vec3(0.30, 0.76, 1.0) * exp(-abs(d - f) * 30.0) * sin(p * 3.14159) * 0.9;
    } else if (uKind == 4) {
        // Minimalista: círculo que se abre desde el centro, borde muy suave
        float d = length((uv - 0.5) * vec2(1.6, 1.0));
        float f = p * 1.2;
        float m = smoothstep(f, f - 0.12, d);
        c = mix(A(uv), B(uv), m);
    } else {
        c = mix(A(uv), B(uv), smoothstep(0.0, 1.0, p));
    }
    fragColor = vec4(c, 1.0);
}
