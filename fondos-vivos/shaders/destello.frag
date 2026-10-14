// Destello que cruza toda la pantalla, por encima de las ventanas, al cambiar de estilo.
// Es transparente (alfa premultiplicado) y no captura el ratón.
// uKind: 0 hacker · 1 cyberpunk · 2 macos · 3 windows · 4 minimalista.

uniform float uProgress;
uniform int uKind;
uniform vec2 uSize;
in vec2 vUv;
out vec4 fragColor;

void main() {
    vec2 uv = vUv;
    float p = clamp(uProgress, 0.0, 1.0);
    float s = sin(p * 3.14159);
    vec3 c = vec3(0.0);
    float a = 0.0;

    if (uKind == 0) {
        // barrido de fósforo verde de arriba abajo, con líneas de monitor
        float y = 1.05 - p * 1.25;
        float line = exp(-abs(uv.y - y) * 120.0);
        float trail = smoothstep(y + 0.35, y, uv.y) * step(y, uv.y);
        float scan = 0.6 + 0.4 * sin(uv.y * uSize.y * 1.5);
        c = vec3(0.0, 1.0, 0.35);
        a = (line * 0.75 + trail * 0.10 * scan) * s + 0.04 * s;
    } else if (uKind == 1) {
        // franjas de glitch magenta y cian que parpadean
        float tick = floor(p * 22.0);
        float band = floor(uv.y * 45.0);
        float on = step(0.86, hash21(vec2(band, tick)));
        float len = hash21(vec2(tick, band * 3.1));
        float x0 = hash21(vec2(band * 7.0, tick));
        float inBar = step(x0, uv.x) * step(uv.x, x0 + len * 0.6);
        c = mix(vec3(1.0, 0.17, 0.84), vec3(0.0, 0.94, 1.0), step(0.5, hash21(vec2(band, tick * 1.9))));
        a = on * inBar * 0.55 * s;
        // golpe de luz inicial y líneas de barrido
        a += 0.18 * exp(-p * 9.0);
        a += 0.06 * s * (0.5 + 0.5 * sin(uv.y * uSize.y * 0.9));
        c = mix(c, vec3(1.0), 0.6 * exp(-p * 9.0));
    } else if (uKind == 2) {
        // macOS: un brillo blanco suave que respira desde el centro
        float d = length((uv - 0.5) * vec2(1.6, 1.0));
        c = vec3(1.0);
        a = 0.22 * s * smoothstep(1.1, 0.0, d);
    } else if (uKind == 3) {
        // Windows: banda de luz azul que cruza en diagonal
        float d = uv.x * 0.75 + (1.0 - uv.y) * 0.25;
        float f = p * 1.4 - 0.2;
        c = vec3(0.30, 0.76, 1.0);
        a = exp(-abs(d - f) * 18.0) * 0.30 * s;
    } else {
        // Minimalista: una línea fina que se abre desde el centro
        float d = length((uv - 0.5) * vec2(1.6, 1.0));
        c = vec3(0.533, 0.753, 0.816);
        a = exp(-abs(d - p * 1.1) * 60.0) * 0.35 * s;
    }
    a = clamp(a, 0.0, 0.85);
    fragColor = vec4(c * a, a);
}
