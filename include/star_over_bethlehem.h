#pragma once

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>
#include <string.h>

#define DEFINE_HANDLE(name) typedef struct name##_t* name

DEFINE_HANDLE(RhiInstance);
DEFINE_HANDLE(RhiAdapter);
DEFINE_HANDLE(RhiDevice);
DEFINE_HANDLE(RhiShader);

typedef int32_t RhiBool;

typedef struct RhiStringView {
    const char* data;
    size_t length;
} RhiStringView;

typedef struct RhiArrayView {
    const void* data;
    size_t size;
} RhiArrayView;

typedef enum {
    RHI_SUCCESS = 0,
    RHI_BACKEND_NOT_SUPPORTED = 1
} RhiResult;

typedef enum {
    RHI_BACKEND_D3D12 = 0,
    RHI_BACKEND_METAL = 1,
    RHI_BACKEND_VULKAN = 2
} RhiBackend;

typedef struct RhiInstanceDesc {
    RhiBool validation;
    RhiBackend backend;
} RhiInstanceDesc;

typedef struct RhiAdapterProperties {
    RhiStringView name;
} RhiAdapterDesc;

typedef struct RhiDeviceDesc {
    RhiAdapter adapter;
} RhiDeviceDesc;

typedef enum RhiShaderStage {
    RHI_SHADER_STAGE_VERTEX = 0,
    RHI_SHADER_STAGE_AMPLIFICATION = 1,
    RHI_SHADER_STAGE_MESH = 2,
    RHI_SHADER_STAGE_PIXEL = 3,
    RHI_SHADER_STAGE_COMPUTE = 4
} RhiShaderStage;

typedef enum RhiShaderSourceType {
    RHI_SHADER_SOURCE_TYPE_HLSL,
    RHI_SHADER_SOURCE_TYPE_BINARY
} RhiShaderSourceType;

typedef union RhiShaderSource {
    struct {
        RhiStringView source;
        RhiArrayView binary;
    };
    RhiShaderSourceType type;
} RhiShaderSource;

typedef struct RhiShaderDesc {
    RhiStringView name;
    RhiShaderSource source;
    RhiShaderStage shader_stage;
} RhiShaderDesc;

RhiResult sob_instance_create(const RhiInstanceDesc* desc, RhiInstance* instance);
void sob_instance_destroy(RhiInstance instance);
void sob_instance_enumerate_adapters(RhiInstance instance, uint32_t* num_adapters, RhiAdapter* adapters);
RhiResult sob_instance_create_device(RhiInstance instance, RhiDeviceDesc* desc, RhiDevice* device);
void sob_destroy_device(RhiDevice device);

RhiResult sob_adapter_get_properties(RhiAdapter adapter, RhiAdapterProperties* properties);

RhiResult sob_device_create_shader(RhiDevice device, RhiShader* shader);

#ifdef __cplusplus
}
#endif