struct MSOutput {
    float4 Position: SV_Position;
    float3 Color: COLOR0;
};

[NumThreads(1, 1, 1)]
[OutputTopology("triangle")]
void ms_main(uint gtid: SV_GroupThreadID, uint gid: SV_GroupID, out indices uint3 triangles[124], out vertices MSOutput vertices[64]) {
    SetMeshOutputCounts(3, 1);
    triangles[0] = uint3(0, 1, 2);

    vertices[0].Position = float4(-0.5, 0.5, 0.0, 1.0);
    vertices[0].Color = float3(1.0, 0.0, 0.0);

    vertices[1].Position = float4(0.5, 0.5, 0.0, 1.0);
    vertices[1].Color = float3(0.0, 1.0, 0.0);

    vertices[2].Position = float4(0.0, -0.5, 0.0, 1.0);
    vertices[2].Color = float3(0.0, 0.0, 1.0);
}