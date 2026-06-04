#include <metal_stdlib>

using namespace metal;

struct VertexOut {
    float4 position [[position]];
    float2 uv;
    uint vertexId;
};

vertex VertexOut vertex_main(uint id [[vertex_id]])
{
    float2 pos[6] = {                                               // esses são os pontos do triangulo
        float2(-1.0, 1.0),  float2(1.0, -1.0), float2(1.0, 1.0),    // triangulo de cima
        float2(-1.0, -1.0), float2(1.0, -1.0), float2(-1.0, 1.0),   // triangulo de baixo
    };

    VertexOut out;
    out.position = float4(pos[id], 0.0, 1.0);
    out.uv = (pos[id] + 1.0) * 0.5;
    out.vertexId = id;
    return out;
}

fragment float4 fragment_main(VertexOut in [[stage_in]], constant float &alpha [[buffer(0)]])
{
    float2 points[6] = {                                            // esses são os pontos do circulo
        float2(0.25, 0.25), float2(0.75, 0.25), float2(0.25, 0.75),
        float2(0.25, 0.75), float2(0.75, 0.25), float2(0.75, 0.75)
    };

    for (uint i = 0; i < 6; i++) {
        if (distance(in.uv, points[i]) < 0.025) {                   // 0.025 refere-se ao ráio, grandeza proporcional ao circulo
            return float4(1.0, 1.0, 0.0, 1.0);                      // marca o vértice
        }
    }

    float g = smoothstep(1.0, 1.0, in.uv.y);
    float r = in.uv.x;
    
    return float4(r, g, 1.0 - r, 0.2);                              // gradiente multicolorido com alpha
}