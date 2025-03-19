struct PSOutput {
    float4 color : SV_Target0;
};

PSOutput fs_main() {
    PSOutput output;
    output.color = float4(0.0, 1.0, 0.0, 1.0);
    return output;
}