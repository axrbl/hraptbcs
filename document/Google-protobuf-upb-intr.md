Google Proto Buffer - upb for C 
=================================


以第1步的简单示例进行简要介绍整个流程

# 1. 编写`.proto`（协议）文件 - device_config.proto
``` protobuf
syntax = "proto2";

message Channel {
  required uint32 channel_id = 1;                       // 通道ID
  optional string channel_name = 2 [default = "udef"];  // 通道名称
  optional float frequency = 3 [default = 5.0];         // 频率
}

message Device {
  optional float version = 5 [default = 1.0];           // 配置版本
  required uint32 device_id = 1;                        // 设备ID
  optional string device_name = 2 [default = "udef"];   // 设备名称
  repeated Channel channels = 3;                        // 包含的通道列表
  optional bool debug_mode = 4 [default = false];       // 调试模式开关
}

```

# 2. 编写对应的配置文件 - device_config.conf

``` protobuf
device_id: 12345
device_name: "qspi"
channels {
  channel_id: 1
  channel_name: "log"
  frequency: 0.3
}
channels {
  channel_id: 2
  channel_name: "audio"
  frequency: 192.00
}
debug_mode: true
firmware_version: 1.2
```

# 3. 下载/构建Protobuf工具
我们的使用场景，直接下载二进制也是可以的；编译/安装也仅需做一次，后续使用即可
``` bash
git clone https://github.com/protocolbuffers/protobuf.git
cd protobuf
# build upb
bazel build //upb/...
# build plug-in<protoc-gen-upb>
bazel build //protoc-gen-upb
# path export
export PATH="$PATH:/usr/local/bin:$(pwd)/bazel-bin"
```
构建完成后，protoc-gen-upb 插件位于 bazel-bin/protoc-gen-upb,因此最后需要导出路径（或使用全路径）使用


# 4. 转换协议文件为对应的C绑定
``` bash
export PATH=$PATH:/path/to/upb/build

protoc --plugin=protoc-gen-upb=protoc-gen-upb --upb_out=. device_pcie.proto
```
自动生成对应proto协议的数据pack/unpack和unpack后的访问API：
* device_config.upb.h
* device_config.upb.c
* channel.upb.h
* channel.upb.c
需要将其加入嵌入式设备的构建，详细使用参加步骤x的C代码示例

注意：upb支持版本兼容，因此老的构建可以使用新版本的协议/配置文件生成的bin，但需注意不要破坏协议的兼容，详情xxx（待进一步补充）

# 5. 编译配置文件为bin文件
``` bash
protoc --encode=DeviceConfig device_config.proto < device_config.txt > device_config.bin
```

# 6. C示例
``` C
#include <stdio.h>
#include <stdlib.h>
#include "upb/upb.h"
#include "channel.upb.h"
#include "device_config.upb.h"
#include "upb/decode.h"

int main() {
    // 假设配置数据存储在某个内存地址
    const char *config_data = /* 指向 device_config.bin 的数据 */;
    size_t config_size = /* 配置数据的大小 */;

    // 创建 UPB 内存分配器
    upb_arena *arena = upb_arena_new();

    // 解析二进制配置数据
    Device *device = Device_parse(config_data, config_size, arena);
    if (!device) {
        fprintf(stderr, "Failed to parse device configuration data.\n");
        upb_arena_free(arena);
        return -1;
    }

    // 访问设备信息
    uint32_t device_id = Device_device_id(device);
    upb_strview device_name = Device_device_name(device);
    bool debug_mode = Device_debug_mode(device);
    float firmware_version = Device_firmware_version(device);

    printf("Device ID: %u\n", device_id);
    printf("Device Name: %.*s\n", (int)device_name.size, device_name.data);
    printf("Debug Mode: %s\n", debug_mode ? "Enabled" : "Disabled");
    printf("Firmware Version: %.1f\n", firmware_version);

    // 访问通道信息
    size_t channel_count;
    const Channel * const *channels = Device_channels(device, &channel_count);

    printf("Channels (%zu):\n", channel_count);
    for (size_t i = 0; i < channel_count; ++i) {
        const Channel *channel = channels[i];
        uint32_t channel_id = Channel_channel_id(channel);
        upb_strview channel_name = Channel_channel_name(channel);
        float frequency = Channel_frequency(channel);

        printf("  Channel %zu:\n", i + 1);
        printf("    ID: %u\n", channel_id);
        printf("    Name: %.*s\n", (int)channel_name.size, channel_name.data);
        printf("    Frequency: %.2f MHz\n", frequency);
    }

    // 释放分配的内存
    upb_arena_free(arena);

    return 0;
}

```


